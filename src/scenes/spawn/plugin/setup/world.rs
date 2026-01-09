use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    DefaultRapierContext, RapierConfiguration, RapierContextSimulation, TimestepMode,
};
use bevy_volumetric_clouds::config::CloudsConfig as CloudsRenderConfig;

use crate::scenes::{
    bounds::SceneBounds,
    config::{ActiveScene, CloudsConfig, RenderConfig, SunConfig},
    entities::EntitiesConfig,
    loaders::{load_entities_config, load_world_config, ConfigLoad, TomlCache},
    spawn::{lights::spawn_lights, sun::spawn_sun, world::spawn_world_entities},
    world::WorldConfig,
    MeshCacheSettings, TomlAsset,
};

pub(crate) fn load_world_and_entities(
    active_scene: &ActiveScene,
    commands: &mut Commands,
    toml_cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> Option<(WorldConfig, EntitiesConfig)> {
    let world_config: WorldConfig = match load_world_config(
        &active_scene.name,
        toml_cache,
        asset_server,
        toml_assets,
    ) {
        ConfigLoad::Pending => return None,
        ConfigLoad::Ready(config) => config,
    };
    let entities_config = match load_entities_config(
        &active_scene.name,
        toml_cache,
        asset_server,
        toml_assets,
    ) {
        ConfigLoad::Pending => return None,
        ConfigLoad::Ready(config) => config,
    };

    commands.insert_resource(SceneBounds::from(world_config.bounds.clone()));
    Some((world_config, entities_config))
}

pub(crate) fn spawn_world_content(
    world_config: &WorldConfig,
    entities_config: &EntitiesConfig,
    active_scene: &ActiveScene,
    mesh_cache: &MeshCacheSettings,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
    toml_cache: &mut TomlCache,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    rapier_context: &mut Query<&mut RapierContextSimulation, With<DefaultRapierContext>>,
) -> bool {
    let world_entities_ready = spawn_world_entities(
        entities_config,
        commands,
        meshes,
        materials,
        asset_server,
        active_scene,
        mesh_cache,
        toml_assets,
        toml_cache,
    );
    if matches!(world_entities_ready, ConfigLoad::Pending) {
        return false;
    }

    spawn_sun(
        world_config.sun.as_ref(),
        commands,
        meshes,
        materials,
        asset_server,
        mesh_cache,
    );

    if let Some(skybox) = world_config.skybox.clone() {
        if let Some(rgb) = crate::scenes::config::parse_color(&skybox.color) {
            commands.insert_resource(ClearColor(Color::srgb_u8(rgb[0], rgb[1], rgb[2])));
        } else {
            warn!(
                "Failed to parse skybox color '{}'; leaving default clear color",
                skybox.color
            );
        }
    }

    if let Ok(mut config) = rapier_config.single_mut() {
        if let Some(gravity) = world_config.gravity.as_ref() {
            config.gravity = Vec3::new(gravity.x, gravity.y, gravity.z);
        }
    }
    if let Ok(mut context) = rapier_context.single_mut() {
        if let Some(physics) = world_config.physics.as_ref() {
            if let Some(iterations) = physics.solver_iterations {
                context.integration_parameters.num_solver_iterations =
                    iterations.max(1) as usize;
            }
        }
    }

    let substeps = world_config
        .physics
        .as_ref()
        .and_then(|physics| physics.substeps)
        .unwrap_or(1)
        .max(1) as usize;
    commands.insert_resource(TimestepMode::Variable {
        max_dt: 1.0 / 60.0,
        time_scale: 1.0,
        substeps,
    });

    apply_clouds_config(world_config.render.as_ref(), world_config.sun.as_ref(), commands);

    spawn_lights(&world_config.lights, commands);
    true
}

fn apply_clouds_config(
    render: Option<&RenderConfig>,
    sun: Option<&SunConfig>,
    commands: &mut Commands,
) {
    let Some(render) = render else { return };
    let Some(clouds) = render.clouds.as_ref() else { return };

    let mut config = CloudsRenderConfig::default();
    config.ui_visible = false;

    let sun_dir_set = apply_clouds_overrides(&mut config, clouds);
    if !sun_dir_set {
        if let Some(sun) = sun {
            let dir = sun_direction_from_time(sun.time);
            config.sun_dir = Vec4::new(dir.x, dir.y, dir.z, 0.0);
        }
    }

    if !clouds.enabled {
        config.clouds_coverage = 0.0;
        config.clouds_density = 0.0;
        config.clouds_detail_strength = 0.0;
    }

    commands.insert_resource(config);
}

fn apply_clouds_overrides(config: &mut CloudsRenderConfig, clouds: &CloudsConfig) -> bool {
    let mut sun_dir_set = false;
    if let Some(value) = clouds.raymarch_steps {
        config.clouds_raymarch_steps_count = value.max(1);
    }
    if let Some(value) = clouds.shadow_raymarch_steps {
        config.clouds_shadow_raymarch_steps_count = value.max(1);
    }
    if let Some(value) = clouds.planet_radius {
        config.planet_radius = value;
    }
    if let Some(value) = clouds.bottom_height {
        config.clouds_bottom_height = value;
    }
    if let Some(value) = clouds.top_height {
        config.clouds_top_height = value;
    }
    if let Some(value) = clouds.coverage {
        config.clouds_coverage = value;
    }
    if let Some(value) = clouds.detail_strength {
        config.clouds_detail_strength = value;
    }
    if let Some(value) = clouds.base_edge_softness {
        config.clouds_base_edge_softness = value;
    }
    if let Some(value) = clouds.bottom_softness {
        config.clouds_bottom_softness = value;
    }
    if let Some(value) = clouds.density {
        config.clouds_density = value;
    }
    if let Some(value) = clouds.shadow_step_size {
        config.clouds_shadow_raymarch_step_size = value;
    }
    if let Some(value) = clouds.shadow_step_multiply {
        config.clouds_shadow_raymarch_step_multiply = value;
    }
    if let Some(value) = clouds.forward_scattering_g {
        config.forward_scattering_g = value;
    }
    if let Some(value) = clouds.backward_scattering_g {
        config.backward_scattering_g = value;
    }
    if let Some(value) = clouds.scattering_lerp {
        config.scattering_lerp = value;
    }
    if let Some(color) = clouds.ambient_color_top.as_deref() {
        config.clouds_ambient_color_top = color_to_vec4(
            color,
            clouds.ambient_alpha_top.unwrap_or(0.0),
            clouds.ambient_intensity_top.unwrap_or(1.0),
        );
    }
    if let Some(color) = clouds.ambient_color_bottom.as_deref() {
        config.clouds_ambient_color_bottom = color_to_vec4(
            color,
            clouds.ambient_alpha_bottom.unwrap_or(0.0),
            clouds.ambient_intensity_bottom.unwrap_or(1.0),
        );
    }
    if let Some(value) = clouds.min_transmittance {
        config.clouds_min_transmittance = value;
    }
    if let Some(value) = clouds.base_scale {
        config.clouds_base_scale = value;
    }
    if let Some(value) = clouds.detail_scale {
        config.clouds_detail_scale = value;
    }
    if let Some(dir) = clouds.sun_direction.as_ref() {
        let direction = Vec3::new(dir.x, dir.y, dir.z);
        if direction.length_squared() > 0.0 {
            let normalized = direction.normalize();
            config.sun_dir = Vec4::new(normalized.x, normalized.y, normalized.z, 0.0);
            sun_dir_set = true;
        }
    }
    let mut sun_color_set = false;
    if let Some(color) = clouds.sun_color.as_deref() {
        config.sun_color = color_to_vec4(
            color,
            clouds.sun_alpha.unwrap_or(1.0),
            clouds.sun_intensity.unwrap_or(1.0),
        );
        sun_color_set = true;
    }
    if !sun_color_set {
        if let Some(alpha) = clouds.sun_alpha {
            config.sun_color.w = alpha.clamp(0.0, 1.0);
        }
        if let Some(intensity) = clouds.sun_intensity {
            config.sun_color *= intensity;
        }
    }
    if let Some(value) = clouds.reprojection_strength {
        config.reprojection_strength = value;
    }
    if let Some(resolution) = clouds.render_resolution.as_ref() {
        config.render_resolution = Vec2::new(resolution.x, resolution.y);
    }
    if let Some(wind) = clouds.wind_velocity.as_ref() {
        config.wind_velocity = Vec3::new(wind.x, wind.y, wind.z);
    }

    sun_dir_set
}

fn color_to_vec4(color: &str, alpha: f32, intensity: f32) -> Vec4 {
    let rgb = crate::scenes::config::parse_color(color).unwrap_or([255, 255, 255]);
    let linear = Color::srgb_u8(rgb[0], rgb[1], rgb[2]).to_linear();
    Vec4::new(linear.red, linear.green, linear.blue, alpha.clamp(0.0, 1.0)) * intensity
}

fn sun_direction_from_time(time: f32) -> Vec3 {
    let fraction = time.rem_euclid(24.0) / 24.0;
    let elevation = (std::f32::consts::PI * fraction).sin().max(0.0);
    Vec3::new(0.0, -(0.1 + elevation), -1.0).normalize()
}
