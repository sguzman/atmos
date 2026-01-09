struct CloudsParams {
    time: f32,
    coverage: f32,
    density: f32,
    raymarch_steps: u32,
    shadow_raymarch_steps: u32,
    use_depth: u32,
    debug_view: u32,
    base_scale: f32,
    detail_scale: f32,
    detail_strength: f32,
    base_edge_softness: f32,
    bottom_softness: f32,
    bottom_height: f32,
    top_height: f32,
    shadow_step_size: f32,
    shadow_step_multiply: f32,
    min_transmittance: f32,
    forward_scattering_g: f32,
    backward_scattering_g: f32,
    scattering_lerp: f32,
    composite_intensity: f32,
    god_rays_intensity: f32,
    ambient_color_top: vec3<f32>,
    ambient_color_bottom: vec3<f32>,
    sun_direction: vec3<f32>,
    wind: vec3<f32>,
    camera_pos: vec3<f32>,
    view_proj_inv: mat4x4<f32>,
};

@group(0) @binding(0)
var clouds_output: texture_storage_2d<rgba16float, write>;
@group(0) @binding(1)
var depth_tex: texture_depth_2d;
@group(0) @binding(2)
var<uniform> params: CloudsParams;

fn hash3(p: vec3<f32>) -> f32 {
    let h = dot(p, vec3<f32>(127.1, 311.7, 74.7));
    return fract(sin(h) * 43758.5453);
}

fn noise3(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    let n000 = hash3(i + vec3<f32>(0.0, 0.0, 0.0));
    let n100 = hash3(i + vec3<f32>(1.0, 0.0, 0.0));
    let n010 = hash3(i + vec3<f32>(0.0, 1.0, 0.0));
    let n110 = hash3(i + vec3<f32>(1.0, 1.0, 0.0));
    let n001 = hash3(i + vec3<f32>(0.0, 0.0, 1.0));
    let n101 = hash3(i + vec3<f32>(1.0, 0.0, 1.0));
    let n011 = hash3(i + vec3<f32>(0.0, 1.0, 1.0));
    let n111 = hash3(i + vec3<f32>(1.0, 1.0, 1.0));

    let nx00 = mix(n000, n100, u.x);
    let nx10 = mix(n010, n110, u.x);
    let nx01 = mix(n001, n101, u.x);
    let nx11 = mix(n011, n111, u.x);
    let nxy0 = mix(nx00, nx10, u.y);
    let nxy1 = mix(nx01, nx11, u.y);
    return mix(nxy0, nxy1, u.z);
}

fn hg_phase(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    let denom = pow(1.0 + g2 - 2.0 * g * cos_theta, 1.5);
    return (1.0 - g2) / max(denom, 1e-4);
}

fn cloud_density(p: vec3<f32>) -> f32 {
    let height = (p.y - params.bottom_height) / max(params.top_height - params.bottom_height, 1.0);
    if (height <= 0.0 || height >= 1.0) {
        return 0.0;
    }

    let height_factor = smoothstep(0.0, params.bottom_softness, height)
        * (1.0 - smoothstep(1.0 - params.base_edge_softness, 1.0, height));

    let wind = params.wind * params.time * 0.05;
    let base = noise3(p * params.base_scale + wind);
    let detail = noise3(p * params.detail_scale - wind * 0.2);
    let n = mix(base, detail, params.detail_strength);
    let coverage = max(params.coverage, 0.001);
    let shaped = clamp((n - (1.0 - coverage)) / coverage, 0.0, 1.0);
    return shaped * height_factor * params.density;
}

fn shadow_transmittance(pos: vec3<f32>, sun_dir: vec3<f32>) -> f32 {
    let steps = max(1.0, f32(params.shadow_raymarch_steps));
    let step_size = max(1.0, params.shadow_step_size);
    var transmittance = 1.0;
    var t = step_size;
    for (var i = 0.0; i < steps; i = i + 1.0) {
        let sample_pos = pos + sun_dir * t;
        let density = cloud_density(sample_pos);
        transmittance *= exp(-density * step_size * params.shadow_step_multiply);
        if (transmittance < 0.02) {
            break;
        }
        t += step_size;
    }
    return clamp(transmittance, 0.0, 1.0);
}

@compute @workgroup_size(8, 8, 1)
fn clouds_compute(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(clouds_output);
    if (id.x >= size.x || id.y >= size.y) {
        return;
    }

    let uv = (vec2<f32>(f32(id.x) + 0.5, f32(id.y) + 0.5) / vec2<f32>(size));
    let ndc = vec2<f32>(uv * 2.0 - 1.0);
    let clip_near = vec4<f32>(ndc, 1.0, 1.0);
    let clip_far = vec4<f32>(ndc, 0.0, 1.0);
    let world_near = params.view_proj_inv * clip_near;
    let world_far = params.view_proj_inv * clip_far;
    let near_pos = world_near.xyz / world_near.w;
    let far_pos = world_far.xyz / world_far.w;
    let ray_dir = normalize(far_pos - near_pos);

    let sampled_depth = textureLoad(depth_tex, vec2<i32>(id.xy), 0);
    let depth_sample = select(1.0, sampled_depth, params.use_depth != 0u);
    var max_distance = 20000.0;
    if (depth_sample > 0.0) {
        let clip = vec4<f32>(ndc, depth_sample, 1.0);
        let world = params.view_proj_inv * clip;
        let world_pos = world.xyz / world.w;
        max_distance = distance(params.camera_pos, world_pos);
    }

    var t_min = 0.0;
    var t_max = max_distance;
    let dir_y = ray_dir.y;
    if (abs(dir_y) > 1e-4) {
        let t0 = (params.bottom_height - params.camera_pos.y) / dir_y;
        let t1 = (params.top_height - params.camera_pos.y) / dir_y;
        t_min = max(min(t0, t1), 0.0);
        t_max = min(max(t0, t1), max_distance);
    } else {
        if (params.camera_pos.y < params.bottom_height || params.camera_pos.y > params.top_height) {
            t_max = 0.0;
        }
    }

    if (t_max <= t_min) {
        textureStore(clouds_output, vec2<i32>(id.xy), vec4<f32>(0.0));
        return;
    }

    let steps = max(8.0, f32(params.raymarch_steps));
    let step_size = (t_max - t_min) / steps;
    var transmittance = 1.0;
    var color = vec3<f32>(0.0);

    let sun_dir = normalize(params.sun_direction);
    for (var i = 0.0; i < steps; i = i + 1.0) {
        let t = t_min + (i + 0.5) * step_size;
        let pos = params.camera_pos + ray_dir * t;
        let density = cloud_density(pos);
        if (density <= 0.0) {
            continue;
        }

        let height = clamp(
            (pos.y - params.bottom_height) / max(params.top_height - params.bottom_height, 1.0),
            0.0,
            1.0
        );
        let ambient = mix(params.ambient_color_bottom, params.ambient_color_top, height);
        let cos_theta = dot(ray_dir, sun_dir);
        let phase_forward = hg_phase(cos_theta, params.forward_scattering_g);
        let phase_backward = hg_phase(cos_theta, params.backward_scattering_g);
        let phase = mix(phase_backward, phase_forward, params.scattering_lerp);
        let shadow = shadow_transmittance(pos, sun_dir);
        let lighting = (ambient + phase * params.god_rays_intensity) * shadow;

        let absorb = exp(-density * step_size * 2.0);
        color += transmittance * lighting * density * step_size * 0.35;
        transmittance *= absorb;
        if (transmittance < params.min_transmittance) {
            break;
        }
    }

    let alpha = clamp(1.0 - transmittance, 0.0, 1.0);
    textureStore(clouds_output, vec2<i32>(id.xy), vec4<f32>(color, alpha));
}

@group(0) @binding(0)
var scene_tex: texture_2d<f32>;
@group(0) @binding(1)
var clouds_tex: texture_2d<f32>;
@group(0) @binding(2)
var scene_sampler: sampler;
@group(0) @binding(3)
var<uniform> composite_params: CloudsParams;

struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@fragment
fn clouds_composite(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let scene = textureSample(scene_tex, scene_sampler, in.uv);
    let clouds = textureSample(clouds_tex, scene_sampler, in.uv);
    if (composite_params.debug_view != 0u) {
        let debug_tint = vec3<f32>(1.0, 0.0, 1.0) * clouds.a;
        return vec4<f32>(scene.rgb + debug_tint, scene.a);
    }
    let color = mix(scene.rgb, scene.rgb + clouds.rgb * composite_params.composite_intensity, clouds.a);
    return vec4<f32>(color, scene.a);
}
