struct CloudsParams {
    time: f32,
    coverage: f32,
    density: f32,
    composite_intensity: f32,
    color: vec4<f32>,
    wind: vec3<f32>,
    god_rays_intensity: f32,
};

@group(0) @binding(0)
var clouds_output: texture_storage_2d<rgba16float, write>;
@group(0) @binding(1)
var<uniform> params: CloudsParams;

fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

@compute @workgroup_size(8, 8, 1)
fn clouds_compute(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(clouds_output);
    if (id.x >= size.x || id.y >= size.y) {
        return;
    }

    let uv = vec2<f32>(f32(id.x) / f32(size.x), f32(id.y) / f32(size.y));
    let wind = params.wind.xy * params.time * 0.02;
    let base = noise(uv * 6.0 + wind);
    let detail = noise(uv * 24.0 - wind * 1.7);
    let n = mix(base, detail, 0.35);
    let coverage = params.coverage;
    let alpha = clamp((n - (1.0 - coverage)) * params.density * 20.0, 0.0, 1.0);
    let color = params.color.rgb * (0.5 + 0.5 * n);

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
    let intensity = composite_params.composite_intensity;
    let color = mix(scene.rgb, scene.rgb + clouds.rgb * intensity, clouds.a);
    return vec4<f32>(color, scene.a);
}
