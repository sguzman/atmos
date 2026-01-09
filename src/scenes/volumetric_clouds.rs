use bevy::asset::load_embedded_asset;
use bevy::core_pipeline::{
    core_3d::graph::{Core3d, Node3d},
    FullscreenShader,
};
use bevy::prelude::*;
use bevy::render::{
    extract_resource::{ExtractResource, ExtractResourcePlugin},
    render_graph::{
        NodeRunError, RenderGraphContext, RenderGraphExt, RenderLabel, ViewNode, ViewNodeRunner,
    },
    render_resource::{
        BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBindingType,
        ColorTargetState, ColorWrites, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d,
        FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPassColorAttachment,
        RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType,
        SamplerDescriptor, ShaderStages, ShaderType, StorageTextureAccess, Texture,
        TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
        TextureView, TextureViewDescriptor, TextureViewDimension, UniformBuffer,
    },
    renderer::{RenderContext, RenderDevice, RenderQueue},
    view::{ExtractedView, ViewDepthTexture, ViewTarget},
    Render, RenderApp, RenderSystems,
};
use bevy_shader::Shader;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::scenes::config::{RenderConfig, SunConfig, VolumetricCloudsConfig};

#[derive(Resource, Debug, Clone, ExtractResource)]
pub struct SceneCloudsConfig {
    pub config: VolumetricCloudsConfig,
    pub sun_direction: Vec3,
}

pub fn apply_clouds_settings(
    render: Option<&RenderConfig>,
    sun: Option<&SunConfig>,
    commands: &mut Commands,
) {
    let config = render
        .and_then(|render| render.clouds.clone())
        .unwrap_or_default();
    commands.insert_resource(SceneCloudsConfig {
        config,
        sun_direction: sun_direction_from_config(sun),
    });
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, RenderLabel)]
pub struct CloudsRenderNodeLabel;

pub struct VolumetricCloudsPlugin;

impl Plugin for VolumetricCloudsPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "../shaders/volumetric_clouds.wgsl");
        app.add_plugins(ExtractResourcePlugin::<SceneCloudsConfig>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<CloudsTextures>();
        render_app.init_resource::<CloudsUniforms>();
        render_app.add_systems(
            Render,
            prepare_clouds_uniforms.in_set(RenderSystems::PrepareResources),
        );
        render_app.add_systems(
            Render,
            init_clouds_pipeline.in_set(RenderSystems::PrepareResources),
        );

        render_app.add_render_graph_node::<ViewNodeRunner<CloudsRenderNode>>(
            Core3d,
            CloudsRenderNodeLabel,
        );
        render_app.add_render_graph_edge(Core3d, Node3d::EndMainPass, CloudsRenderNodeLabel);
        render_app.add_render_graph_edge(
            Core3d,
            CloudsRenderNodeLabel,
            Node3d::StartMainPassPostProcessing,
        );
    }
}

#[derive(Clone, Copy, ShaderType, Default)]
struct CloudsParams {
    time: f32,
    coverage: f32,
    density: f32,
    raymarch_steps: u32,
    shadow_raymarch_steps: u32,
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
    ambient_color_top: Vec3,
    ambient_color_bottom: Vec3,
    sun_direction: Vec3,
    wind: Vec3,
    camera_pos: Vec3,
    view_proj_inv: Mat4,
}

#[derive(Resource, Default)]
struct CloudsUniforms {
    buffer: UniformBuffer<CloudsParams>,
}

#[derive(Default)]
struct CloudsTexturesInner {
    size: UVec2,
    texture: Option<Texture>,
    view: Option<TextureView>,
}

#[derive(Resource, Default)]
struct CloudsTextures {
    inner: Mutex<CloudsTexturesInner>,
}

#[derive(Resource)]
struct CloudsPipeline {
    shader: Handle<Shader>,
    compute_layout: BindGroupLayout,
    render_layout: BindGroupLayout,
    sampler: Sampler,
    compute_pipeline: bevy::render::render_resource::CachedComputePipelineId,
    render_pipelines:
        Mutex<HashMap<TextureFormat, bevy::render::render_resource::CachedRenderPipelineId>>,
    fullscreen_shader: FullscreenShader,
}

impl FromWorld for CloudsPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let fullscreen_shader = world.resource::<FullscreenShader>().clone();

        let shader = load_embedded_asset!(asset_server, "../shaders/volumetric_clouds.wgsl");

        let compute_layout = render_device.create_bind_group_layout(
            "clouds_compute_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba16Float,
                        view_dimension: TextureViewDimension::D2,
                    },
                    BindingType::Texture {
                        sample_type: TextureSampleType::Depth,
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
            ),
        );

        let render_layout = render_device.create_bind_group_layout(
            "clouds_render_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    BindingType::Sampler(SamplerBindingType::Filtering),
                    BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let compute_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("volumetric_clouds_compute".into()),
            layout: vec![compute_layout.clone()],
            shader: shader.clone(),
            shader_defs: Vec::new(),
            entry_point: Some("clouds_compute".into()),
            push_constant_ranges: Vec::new(),
            zero_initialize_workgroup_memory: true,
        });

        Self {
            shader,
            compute_layout,
            render_layout,
            sampler,
            compute_pipeline,
            render_pipelines: Mutex::new(HashMap::default()),
            fullscreen_shader,
        }
    }
}

fn init_clouds_pipeline(
    mut commands: Commands,
    pipeline: Option<Res<CloudsPipeline>>,
    render_device: Option<Res<RenderDevice>>,
) {
    if pipeline.is_some() || render_device.is_none() {
        return;
    }
    commands.init_resource::<CloudsPipeline>();
}

#[derive(Default)]
struct CloudsRenderNode;

impl ViewNode for CloudsRenderNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static ExtractedView,
        &'static ViewDepthTexture,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (target, view, depth): bevy::ecs::query::QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let config = world.resource::<SceneCloudsConfig>();
        if !config.config.enabled {
            return Ok(());
        }

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<CloudsPipeline>();
        let render_device = world.resource::<RenderDevice>();
        let uniforms = world.resource::<CloudsUniforms>();
        let textures = world.resource::<CloudsTextures>();
        let mut textures_guard = textures.inner.lock().unwrap();

        let size = cloud_render_size(view, &config.config);
        ensure_clouds_texture(render_device, &mut textures_guard, size);

        let Some(clouds_view) = textures_guard.view.as_ref() else {
            return Ok(());
        };
        let depth_view = depth.view();
        let Some(uniform_binding) = uniforms.buffer.binding() else {
            return Ok(());
        };

        let compute_pipeline = match pipeline_cache.get_compute_pipeline(pipeline.compute_pipeline)
        {
            Some(pipeline) => pipeline,
            None => return Ok(()),
        };

        let compute_bind_group = render_device.create_bind_group(
            "clouds_compute_bind_group",
            &pipeline.compute_layout,
            &BindGroupEntries::sequential((clouds_view, depth_view, uniform_binding.clone())),
        );

        let mut compute_pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor {
                label: Some("volumetric_clouds_compute"),
                timestamp_writes: None,
            });
        compute_pass.set_pipeline(compute_pipeline);
        compute_pass.set_bind_group(0, &compute_bind_group, &[]);
        let workgroup_x = (size.x + 7) / 8;
        let workgroup_y = (size.y + 7) / 8;
        compute_pass.dispatch_workgroups(workgroup_x, workgroup_y, 1);
        drop(compute_pass);

        let render_pipeline_id = get_or_create_render_pipeline(
            pipeline_cache,
            pipeline,
            target.main_texture_format(),
        );
        let Some(render_pipeline) = pipeline_cache.get_render_pipeline(render_pipeline_id) else {
            return Ok(());
        };

        let post_process = target.post_process_write();
        let source = post_process.source;
        let destination = post_process.destination;

        let render_bind_group = render_device.create_bind_group(
            "clouds_render_bind_group",
            &pipeline.render_layout,
            &BindGroupEntries::sequential((
                source,
                clouds_view,
                &pipeline.sampler,
                uniform_binding,
            )),
        );

        let mut render_pass = render_context
            .command_encoder()
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("volumetric_clouds_composite"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: destination,
                    depth_slice: None,
                    resolve_target: None,
                    ops: bevy::render::render_resource::Operations {
                        load: bevy::render::render_resource::LoadOp::Load,
                        store: bevy::render::render_resource::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        render_pass.set_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &render_bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

fn ensure_clouds_texture(
    render_device: &RenderDevice,
    textures: &mut CloudsTexturesInner,
    size: UVec2,
) {
    if textures.texture.is_some() && textures.size == size {
        return;
    }

    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("volumetric_clouds_texture"),
        size: Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Float,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&TextureViewDescriptor::default());

    textures.size = size;
    textures.texture = Some(texture);
    textures.view = Some(view);
}

fn get_or_create_render_pipeline(
    pipeline_cache: &PipelineCache,
    pipeline: &CloudsPipeline,
    format: TextureFormat,
) -> bevy::render::render_resource::CachedRenderPipelineId {
    let mut pipelines = pipeline.render_pipelines.lock().unwrap();
    let cached = pipelines
        .get(&format)
        .copied()
        .unwrap_or_else(|| pipeline_cache.queue_render_pipeline(build_render_pipeline(
            pipeline,
            format,
        )));
    pipelines.insert(format, cached);
    cached
}

fn build_render_pipeline(pipeline: &CloudsPipeline, format: TextureFormat) -> RenderPipelineDescriptor {
    RenderPipelineDescriptor {
        label: Some("volumetric_clouds_render".into()),
        layout: vec![pipeline.render_layout.clone()],
        vertex: pipeline.fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader: pipeline.shader.clone(),
            shader_defs: Vec::new(),
            entry_point: Some("clouds_composite".into()),
            targets: vec![Some(ColorTargetState {
                format,
                blend: Some(bevy::render::render_resource::BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: Vec::new(),
        zero_initialize_workgroup_memory: true,
    }
}

fn cloud_render_size(view: &ExtractedView, config: &VolumetricCloudsConfig) -> UVec2 {
    let viewport_size = UVec2::new(view.viewport.z.max(1), view.viewport.w.max(1));
    let Some(target) = config.render_resolution.as_ref() else {
        return viewport_size;
    };
    let requested = UVec2::new(
        target.x.max(1.0).round() as u32,
        target.y.max(1.0).round() as u32,
    );
    UVec2::new(
        requested.x.min(viewport_size.x),
        requested.y.min(viewport_size.y),
    )
}

fn update_clouds_uniforms(
    time: &Time,
    view: &ExtractedView,
    cloud_cfg: &VolumetricCloudsConfig,
    sun_direction: Vec3,
    uniforms: &mut CloudsUniforms,
    render_device: &RenderDevice,
    render_queue: &RenderQueue,
) {
    let coverage = cloud_cfg.coverage.unwrap_or(0.5);
    let density = cloud_cfg.density.unwrap_or(0.03);
    let base_scale = cloud_cfg.base_scale.unwrap_or(1.5);
    let detail_scale = cloud_cfg.detail_scale.unwrap_or(20.0);
    let detail_strength = cloud_cfg.detail_strength.unwrap_or(0.35);
    let raymarch_steps = cloud_cfg.raymarch_steps.unwrap_or(64);
    let shadow_raymarch_steps = cloud_cfg.shadow_raymarch_steps.unwrap_or(12);
    let shadow_step_size = cloud_cfg.shadow_step_size.unwrap_or(12.0);
    let shadow_step_multiply = cloud_cfg.shadow_step_multiply.unwrap_or(1.25);
    let base_edge_softness = cloud_cfg.base_edge_softness.unwrap_or(0.12);
    let bottom_softness = cloud_cfg.bottom_softness.unwrap_or(0.22);
    let bottom_height = cloud_cfg.bottom_height.unwrap_or(1200.0);
    let top_height = cloud_cfg.top_height.unwrap_or(2400.0);
    let min_transmittance = cloud_cfg.min_transmittance.unwrap_or(0.1);
    let forward_scattering_g = cloud_cfg.forward_scattering_g.unwrap_or(0.6);
    let backward_scattering_g = cloud_cfg.backward_scattering_g.unwrap_or(-0.2);
    let scattering_lerp = cloud_cfg.scattering_lerp.unwrap_or(0.5);
    let composite_intensity = cloud_cfg
        .god_rays
        .as_ref()
        .and_then(|rays| rays.intensity)
        .unwrap_or(0.6);

    let wind = cloud_cfg.wind_velocity.clone().unwrap_or_default();
    let god_rays_intensity = cloud_cfg
        .god_rays
        .as_ref()
        .and_then(|rays| if rays.enabled { rays.intensity } else { None })
        .unwrap_or(0.0);
    let ambient_top = cloud_cfg
        .ambient_color_top
        .as_deref()
        .and_then(crate::scenes::config::parse_color)
        .unwrap_or([160, 176, 208]);
    let ambient_bottom = cloud_cfg
        .ambient_color_bottom
        .as_deref()
        .and_then(crate::scenes::config::parse_color)
        .unwrap_or([64, 80, 96]);
    let ambient_top = Color::srgb_u8(ambient_top[0], ambient_top[1], ambient_top[2]).to_linear();
    let ambient_bottom =
        Color::srgb_u8(ambient_bottom[0], ambient_bottom[1], ambient_bottom[2]).to_linear();

    let sun_dir = sun_direction.normalize_or_zero();
    let world_from_view = view.world_from_view.to_matrix();
    let view_from_world = world_from_view.inverse();
    let clip_from_world = view
        .clip_from_world
        .unwrap_or_else(|| view.clip_from_view * view_from_world);
    let view_proj_inv = clip_from_world.inverse();
    let camera_pos = view.world_from_view.translation();

    uniforms.buffer.set(CloudsParams {
        time: time.elapsed_secs(),
        coverage,
        density,
        raymarch_steps,
        shadow_raymarch_steps,
        base_scale,
        detail_scale,
        detail_strength,
        base_edge_softness,
        bottom_softness,
        bottom_height,
        top_height,
        shadow_step_size,
        shadow_step_multiply,
        min_transmittance,
        forward_scattering_g,
        backward_scattering_g,
        scattering_lerp,
        composite_intensity,
        god_rays_intensity,
        ambient_color_top: Vec3::new(ambient_top.red, ambient_top.green, ambient_top.blue),
        ambient_color_bottom: Vec3::new(
            ambient_bottom.red,
            ambient_bottom.green,
            ambient_bottom.blue,
        ),
        sun_direction: sun_dir,
        wind: Vec3::new(wind.x, wind.y, wind.z),
        camera_pos,
        view_proj_inv,
    });
    uniforms
        .buffer
        .write_buffer(render_device, render_queue);
}

fn sun_direction_from_config(sun: Option<&SunConfig>) -> Vec3 {
    let Some(sun) = sun else {
        return Vec3::new(0.3, -0.8, 0.2);
    };
    let fraction = (sun.time.rem_euclid(24.0)) / 24.0;
    let elevation = (std::f32::consts::PI * fraction).sin().max(0.0);
    Vec3::new(0.0, -(0.1 + elevation), -1.0).normalize()
}

fn prepare_clouds_uniforms(
    time: Res<Time>,
    config: Option<Res<SceneCloudsConfig>>,
    mut uniforms: ResMut<CloudsUniforms>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    views: Query<&ExtractedView>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(view) = views.iter().next() else {
        return;
    };
    update_clouds_uniforms(
        &time,
        view,
        &config.config,
        config.sun_direction,
        &mut uniforms,
        &render_device,
        &render_queue,
    );
}
