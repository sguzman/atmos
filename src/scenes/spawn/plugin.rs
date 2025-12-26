use bevy::{
    log::{info, warn},
    prelude::*,
};
use bevy::camera::Exposure;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::render::view::Hdr;

use crate::app_config::AppConfig;
use crate::scenes::{
    bounds::{despawn_out_of_bounds, SceneBounds},
    config::{ActiveScene, BloomConfig, FogConfig, FogFalloffConfig, InputConfig, RenderConfig},
    input::{
        apply_camera_input, apply_fov_action, apply_shoot_action, apply_sprint_toggle,
        apply_zoom_action, resolve_camera_input_config, resolve_overlay_toggles, FovBinding,
        SceneCamera, SceneFovConfig, SceneInputConfig, SceneShootConfig, SceneSprintConfig,
        SceneZoomConfig, SprintState, ZoomState,
    },
    loaders::{
        load_entity_template_from_path, load_input_config, load_shoot_action_config,
        load_sprint_action_config, load_world_config, load_zoom_action_config,
    },
    world::WorldConfig,
};

use super::lights::spawn_lights;
use super::logging::{log_camera, log_lights};
use super::overlay::{spawn_overlays_from_config, OverlayTag};
use super::sun::spawn_sun;
use super::world::spawn_world_entities;

pub struct ScenePlugin {
    scene: &'static str,
}

impl ScenePlugin {
    pub const fn new(scene: &'static str) -> Self {
        Self { scene }
    }
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveScene {
            name: self.scene.to_string(),
        });
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, apply_camera_input);
        app.add_systems(Update, apply_fov_action);
        app.add_systems(Update, apply_shoot_action);
        app.add_systems(Update, apply_sprint_toggle);
        app.add_systems(Update, apply_zoom_action);
        app.add_systems(Update, despawn_out_of_bounds);
        app.add_systems(
            PostStartup,
            (log_lights, log_camera, spawn_overlays_from_config).after(setup_scene),
        );
        app.add_systems(Update, toggle_overlays);
    }
}

fn setup_scene(
    active_scene: Res<ActiveScene>,
    app_config: Res<AppConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!(
        "Bootstrapping scene '{}' with inspector overlay enabled.",
        active_scene.name
    );

    let input_config: InputConfig = load_input_config(&active_scene.name);
    let camera_input =
        resolve_camera_input_config(&input_config.camera.movement, &input_config.camera.rotation);
    commands.insert_resource(SceneInputConfig {
        camera: camera_input,
        overlays: resolve_overlay_toggles(&input_config.overlays),
    });

    let world_config: WorldConfig = load_world_config(&active_scene.name);

    commands.insert_resource(SceneBounds::from(world_config.bounds.clone()));

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("shoot-balls.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_mouse_button_or_warn(&action_binding.mouse, "shoot")
        {
            if let Some(action) =
                load_shoot_action_config(&active_scene.name, &action_binding.action)
            {
                let Some(projectile) = load_entity_template_from_path(
                    &active_scene.name,
                    "entities/sphere.3D.toml",
                ) else {
                    warn!("Projectile template missing; shoot action disabled.");
                    return;
                };

                let Some(shape) = projectile.shape.clone() else {
                    warn!("Projectile template has no shape; shoot action disabled.");
                    return;
                };
                if shape.kind != crate::scenes::config::ShapeKind::Sphere {
                    warn!("Projectile template is not a sphere; shoot action disabled.");
                    return;
                }
                let radius = shape.radius.unwrap_or(0.2);
                let color = shape.color.as_deref().and_then(crate::scenes::config::parse_color)
                    .unwrap_or([255, 165, 0]);
                let sphere_material = materials.add(Color::srgb_u8(color[0], color[1], color[2]));
                let sphere_mesh = meshes.add(Sphere::new(radius));
                commands.insert_resource(SceneShootConfig {
                    action,
                    trigger,
                    name: projectile.name.clone(),
                    shape,
                    physics: projectile.physics.clone(),
                    mesh: sphere_mesh,
                    material: sphere_material,
                });
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("sprint.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "sprint")
        {
            if let Some(action) =
                load_sprint_action_config(&active_scene.name, &action_binding.action)
            {
                commands.insert_resource(SceneSprintConfig { action, trigger });
                commands.insert_resource(SprintState::default());
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("zoom.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "zoom")
        {
            if let Some(action) =
                load_zoom_action_config(&active_scene.name, &action_binding.action)
            {
                commands.insert_resource(SceneZoomConfig { action, trigger });
                commands.insert_resource(ZoomState::default());
            }
        }
    }

    let mut fov_bindings = Vec::new();
    for action_binding in input_config
        .actions
        .iter()
        .filter(|action| action.action.ends_with("fov.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "fov")
        {
            if let Some(fov_value) = action_binding.value {
                fov_bindings.push(FovBinding {
                    trigger,
                    fov_degrees: fov_value,
                });
            } else {
                warn!(
                    "Fov action '{}' is missing a value; binding skipped.",
                    action_binding.name
                );
            }
        }
    }

    if !fov_bindings.is_empty() {
        commands.insert_resource(SceneFovConfig {
            bindings: fov_bindings,
        });
    }

    spawn_world_entities(
        &world_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    // sun derived from world config
    spawn_sun(
        world_config.sun.as_ref(),
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    // skybox clear color
    if let Some(skybox) = world_config.skybox.clone() {
        if let Some(rgb) = crate::scenes::config::parse_color(&skybox.color) {
            commands.insert_resource(ClearColor(Color::srgb_u8(rgb[0], rgb[1], rgb[2])));
        } else {
            warn!("Failed to parse skybox color '{}'; leaving default clear color", skybox.color);
        }
    }

    // lights
    spawn_lights(&world_config.lights, &mut commands);

    // camera
    let camera_components = (
        Name::new(world_config.camera.name.clone()),
        Camera3d::default(),
        SceneCamera,
        Transform::from_xyz(
            world_config.camera.transform.position.x,
            world_config.camera.transform.position.y,
            world_config.camera.transform.position.z,
        )
        .looking_at(
            Vec3::new(
                world_config.camera.transform.look_at.x,
                world_config.camera.transform.look_at.y,
                world_config.camera.transform.look_at.z,
            ),
            Vec3::new(
                world_config.camera.transform.up.x,
                world_config.camera.transform.up.y,
                world_config.camera.transform.up.z,
            ),
        ),
    );
    let mut camera = commands.spawn(camera_components);
    if let Some(msaa) = app_config.msaa_component() {
        camera.insert(msaa);
    }
    if let Some(render) = world_config.render.as_ref() {
        apply_render_settings(&mut camera, render);
    }

    // UI overlay camera
    if let Some(msaa) = app_config.msaa_component() {
        commands.spawn((Camera2d::default(), Camera { order: 1, ..default() }, msaa));
    } else {
        commands.spawn((Camera2d::default(), Camera { order: 1, ..default() }));
    }
}

fn toggle_overlays(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneInputConfig>>,
    mut overlays: Query<(&OverlayTag, &mut Visibility)>,
) {
    let Some(config) = config else { return };
    for overlay in &config.overlays {
        let Some(key) = overlay.toggle else { continue };
        if keys.just_pressed(key) {
            for (_tag, mut vis) in overlays
                .iter_mut()
                .filter(|(tag, _)| tag.name == overlay.name)
            {
                vis.toggle_visible_hidden();
            }
        }
    }
}

fn apply_render_settings(camera: &mut EntityCommands, render: &RenderConfig) {
    let bloom_enabled = render.bloom.as_ref().is_some_and(|bloom| bloom.enabled);
    if render.hdr.unwrap_or(false) || bloom_enabled {
        camera.insert(Hdr);
    }

    if let Some(tonemapping) = render
        .tonemapping
        .as_deref()
        .and_then(parse_tonemapping)
    {
        camera.insert(tonemapping);
    }

    if let Some(ev100) = render.exposure_ev100 {
        camera.insert(Exposure { ev100 });
    }

    if let Some(enabled) = render.deband_dither {
        let dither = if enabled {
            DebandDither::Enabled
        } else {
            DebandDither::Disabled
        };
        camera.insert(dither);
    }

    if let Some(bloom) = render.bloom.as_ref().filter(|bloom| bloom.enabled) {
        camera.insert(resolve_bloom(bloom));
    }

    if let Some(fog) = render.fog.as_ref().filter(|fog| fog.enabled) {
        camera.insert(resolve_fog(fog));
    }
}

fn parse_tonemapping(value: &str) -> Option<Tonemapping> {
    let normalized = value.trim().to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        "none" => Some(Tonemapping::None),
        "reinhard" => Some(Tonemapping::Reinhard),
        "reinhard_luminance" => Some(Tonemapping::ReinhardLuminance),
        "aces_fitted" => Some(Tonemapping::AcesFitted),
        "agx" => Some(Tonemapping::AgX),
        "somewhat_boring_display_transform" => {
            Some(Tonemapping::SomewhatBoringDisplayTransform)
        }
        "tony_mc_mapface" => Some(Tonemapping::TonyMcMapface),
        "blender_filmic" => Some(Tonemapping::BlenderFilmic),
        _ => None,
    }
}

fn resolve_bloom(config: &BloomConfig) -> Bloom {
    let mut bloom = match config
        .preset
        .as_deref()
        .map(|preset| preset.trim().to_ascii_lowercase().replace('-', "_"))
        .as_deref()
    {
        Some("natural") => Bloom::NATURAL,
        Some("old_school") => Bloom::OLD_SCHOOL,
        Some("screen_blur") => Bloom::SCREEN_BLUR,
        _ => Bloom::default(),
    };

    if let Some(value) = config.intensity {
        bloom.intensity = value;
    }
    if let Some(value) = config.low_frequency_boost {
        bloom.low_frequency_boost = value;
    }
    if let Some(value) = config.low_frequency_boost_curvature {
        bloom.low_frequency_boost_curvature = value;
    }
    if let Some(value) = config.high_pass_frequency {
        bloom.high_pass_frequency = value;
    }
    if let Some(prefilter) = config.prefilter.as_ref() {
        bloom.prefilter = BloomPrefilter {
            threshold: prefilter.threshold,
            threshold_softness: prefilter.threshold_softness,
        };
    }
    if let Some(mode) = config
        .composite_mode
        .as_deref()
        .map(|mode| mode.trim().to_ascii_lowercase().replace('-', "_"))
    {
        bloom.composite_mode = match mode.as_str() {
            "additive" => BloomCompositeMode::Additive,
            _ => BloomCompositeMode::EnergyConserving,
        };
    }
    if let Some(value) = config.max_mip_dimension {
        bloom.max_mip_dimension = value;
    }
    if let Some(scale) = config.scale.as_ref() {
        bloom.scale = Vec2::new(scale.x, scale.y);
    }

    bloom
}

fn resolve_fog(config: &FogConfig) -> DistanceFog {
    let color = config
        .color
        .as_deref()
        .and_then(crate::scenes::config::parse_color)
        .unwrap_or([255, 255, 255]);
    let alpha = config.alpha.unwrap_or(1.0).clamp(0.0, 1.0);
    let fog_color = Color::srgba_u8(color[0], color[1], color[2], (alpha * 255.0) as u8);

    let directional_light_color = if let Some(color) = config
        .directional_light_color
        .as_deref()
        .and_then(crate::scenes::config::parse_color)
    {
        let alpha = config
            .directional_light_alpha
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        Color::srgba_u8(color[0], color[1], color[2], (alpha * 255.0) as u8)
    } else {
        Color::NONE
    };

    let falloff = match config.falloff.as_ref() {
        Some(FogFalloffConfig::Linear { start, end }) => FogFalloff::Linear {
            start: *start,
            end: *end,
        },
        Some(FogFalloffConfig::Exponential { density }) => {
            FogFalloff::Exponential { density: *density }
        }
        Some(FogFalloffConfig::ExponentialSquared { density }) => {
            FogFalloff::ExponentialSquared { density: *density }
        }
        Some(FogFalloffConfig::Atmospheric {
            extinction,
            inscattering,
        }) => FogFalloff::Atmospheric {
            extinction: Vec3::new(extinction.x, extinction.y, extinction.z),
            inscattering: Vec3::new(inscattering.x, inscattering.y, inscattering.z),
        },
        None => FogFalloff::Linear { start: 0.0, end: 100.0 },
    };

    DistanceFog {
        color: fog_color,
        directional_light_color,
        directional_light_exponent: config.directional_light_exponent.unwrap_or(8.0),
        falloff,
    }
}
