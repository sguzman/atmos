use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::app_config::AppConfig;
use crate::scenes::{
    bounds::{despawn_out_of_bounds, SceneBounds},
    config::{ActiveScene, CameraConfig, InputConfig},
    input::{
        apply_camera_input, apply_fov_action, apply_shoot_action, apply_sprint_toggle,
        apply_zoom_action, resolve_action_bindings, resolve_camera_input_config,
        resolve_overlay_toggles, FovBinding, SceneCamera, SceneFovConfig, SceneInputConfig,
        SceneShootConfig, SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState,
    },
    loaders::{
        load_bounding_box_config, load_camera_config, load_entity_template_from_path,
        load_fov_action_config, load_input_config, load_light_config,
        load_shoot_action_config, load_skybox_config, load_sprint_action_config, load_sun_config,
        load_world_config, load_zoom_action_config,
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
        actions: resolve_action_bindings(&input_config.actions),
    });

    let camera_config: CameraConfig = load_camera_config(&active_scene.name);
    let bounds_config = load_bounding_box_config(&active_scene.name);
    let world_config: WorldConfig = load_world_config(&active_scene.name);
    let light_config = load_light_config(&active_scene.name);
    let sun_config = load_sun_config(&active_scene.name);
    let skybox_config = load_skybox_config(&active_scene.name);

    commands.insert_resource(SceneBounds::from(bounds_config));

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
    let mut fov_action = None;
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
        if fov_action.is_none() {
            fov_action =
                load_fov_action_config(&active_scene.name, &action_binding.action);
        }
    }

    if let Some(action) = fov_action {
        if !fov_bindings.is_empty() {
            commands.insert_resource(SceneFovConfig {
                action,
                bindings: fov_bindings,
            });
        }
    }

    spawn_world_entities(
        &world_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    // sun derived from world config
    spawn_sun(sun_config.as_ref(), &mut commands, &mut meshes, &mut materials, &active_scene);

    // skybox clear color
    if let Some(skybox) = skybox_config {
        if let Some(rgb) = crate::scenes::config::parse_color(&skybox.color) {
            commands.insert_resource(ClearColor(Color::srgb_u8(rgb[0], rgb[1], rgb[2])));
        } else {
            warn!("Failed to parse skybox color '{}'; leaving default clear color", skybox.color);
        }
    }

    // lights
    spawn_lights(&light_config, &mut commands);

    // camera
    let camera_components = (
        Name::new(camera_config.name),
        Camera3d::default(),
        SceneCamera,
        Transform::from_xyz(
            camera_config.transform.position.x,
            camera_config.transform.position.y,
            camera_config.transform.position.z,
        )
        .looking_at(
            Vec3::new(
                camera_config.transform.look_at.x,
                camera_config.transform.look_at.y,
                camera_config.transform.look_at.z,
            ),
            Vec3::new(
                camera_config.transform.up.x,
                camera_config.transform.up.y,
                camera_config.transform.up.z,
            ),
        ),
    );
    if let Some(msaa) = app_config.msaa_component() {
        commands.spawn((camera_components, msaa));
    } else {
        commands.spawn(camera_components);
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
