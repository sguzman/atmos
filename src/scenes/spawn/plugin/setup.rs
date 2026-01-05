use bevy::camera::{CameraOutputMode, ClearColorConfig};
use bevy::prelude::*;
use bevy::render::render_resource::BlendState;
use bevy_rapier3d::prelude::{
    Collider, DefaultRapierContext, GravityScale, LockedAxes, RapierConfiguration, RigidBody,
    Velocity,
};

use crate::app_config::AppConfig;
use crate::scenes::{
    bounds::SceneBounds,
    config::{ActiveScene, InputConfig},
    input::{
        resolve_camera_input_config, resolve_overlay_toggles, CameraLookState, FovBinding,
        GrabHover, GrabState, NoclipState, PlayerBody, PlayerSpawn, SceneCamera, SceneFovConfig,
        SceneGrabConfig, SceneInputConfig, SceneJumpConfig, SceneNoclipConfig, SceneShootConfig,
        SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState,
    },
    loaders::{
        load_entity_template_from_path, load_entities_config, load_grab_action_config,
        load_input_config, load_jump_action_config, load_noclip_action_config,
        load_shoot_action_config, load_sprint_action_config, load_world_config,
        load_zoom_action_config,
    },
    world::WorldConfig,
};

use super::render::apply_render_settings;
use crate::scenes::spawn::lights::spawn_lights;
use crate::scenes::spawn::sun::spawn_sun;
use crate::scenes::spawn::world::spawn_world_entities;

pub(crate) fn setup_scene(
    active_scene: Res<ActiveScene>,
    app_config: Res<AppConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
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

    let mut initial_noclip = None;

    let world_config: WorldConfig = load_world_config(&active_scene.name);
    let _scene_type = world_config.scene_type.as_deref();
    let entities_config = load_entities_config(&active_scene.name);

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
                let color = shape
                    .color
                    .as_deref()
                    .and_then(crate::scenes::config::parse_color)
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

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("jump.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "jump")
        {
            if let Some(action) =
                load_jump_action_config(&active_scene.name, &action_binding.action)
            {
                commands.insert_resource(SceneJumpConfig { action, trigger });
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("noclip.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "noclip")
        {
            if let Some(action) =
                load_noclip_action_config(&active_scene.name, &action_binding.action)
            {
                let state = NoclipState {
                    active: action.enabled,
                    velocity: Vec3::ZERO,
                    fast: false,
                };
                let speed_toggle_key = crate::scenes::input::resolve_key_or_warn(
                    &action.speed_toggle_key,
                    "noclip speed toggle",
                );
                let up_key =
                    crate::scenes::input::resolve_key_or_warn(&action.up_key, "noclip up");
                let down_key =
                    crate::scenes::input::resolve_key_or_warn(&action.down_key, "noclip down");
                initial_noclip = Some(action.enabled);
                commands.insert_resource(SceneNoclipConfig {
                    action,
                    trigger,
                    speed_toggle_key,
                    up_key,
                    down_key,
                });
                commands.insert_resource(state);
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("grab.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "grab")
        {
            if let Some(action) = load_grab_action_config(&active_scene.name, &action_binding.action)
            {
                let rgb = crate::scenes::config::parse_color(&action.outline.color)
                    .unwrap_or([0, 255, 255]);
                let outline_color = Color::srgb_u8(rgb[0], rgb[1], rgb[2]);
                commands.insert_resource(SceneGrabConfig {
                    action,
                    trigger,
                    outline_color,
                });
                commands.insert_resource(GrabState::default());
                commands.insert_resource(GrabHover::default());
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
        &entities_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
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
            warn!(
                "Failed to parse skybox color '{}'; leaving default clear color",
                skybox.color
            );
        }
    }

    if let Some(gravity) = world_config.gravity.as_ref() {
        if let Ok(mut config) = rapier_config.single_mut() {
            config.gravity = Vec3::new(gravity.x, gravity.y, gravity.z);
        }
    }

    // lights
    spawn_lights(&world_config.lights, &mut commands);

    // player body + camera
    let camera_position = Vec3::new(
        world_config.camera.transform.position.x,
        world_config.camera.transform.position.y,
        world_config.camera.transform.position.z,
    );
    let camera_look_at = Vec3::new(
        world_config.camera.transform.look_at.x,
        world_config.camera.transform.look_at.y,
        world_config.camera.transform.look_at.z,
    );
    let camera_up = Vec3::new(
        world_config.camera.transform.up.x,
        world_config.camera.transform.up.y,
        world_config.camera.transform.up.z,
    );
    let basis = Transform::from_translation(camera_position).looking_at(camera_look_at, camera_up);
    let (yaw, pitch, _) = basis.rotation.to_euler(EulerRot::YXZ);
    let pitch = pitch.clamp(-1.4, 1.4);

    commands.insert_resource(PlayerSpawn {
        position: camera_position,
    });
    commands.insert_resource(CameraLookState { pitch });

    let body_half = Vec3::splat(0.5);
    let (body_type, gravity_scale) = if initial_noclip.unwrap_or(false) {
        (RigidBody::KinematicPositionBased, 0.0)
    } else {
        (RigidBody::Dynamic, 1.0)
    };
    let body_id = commands
        .spawn((
            Name::new(format!("{}_body", world_config.camera.name)),
            PlayerBody,
            body_type,
            Collider::cuboid(body_half.x, body_half.y, body_half.z),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(gravity_scale),
            Velocity::default(),
            Transform::from_translation(camera_position)
                .with_rotation(Quat::from_rotation_y(yaw)),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    let camera_components = (
        Name::new(world_config.camera.name.clone()),
        Camera3d::default(),
        SceneCamera,
        Transform::from_translation(Vec3::new(0.0, 0.6, 0.0))
            .with_rotation(Quat::from_rotation_x(pitch)),
    );
    let camera_id = {
        let mut camera = commands.spawn(camera_components);
        if let Some(msaa) = app_config.msaa_component() {
            camera.insert(msaa);
        }
        if let Some(render) = world_config.render.as_ref() {
            apply_render_settings(&mut camera, render);
        }
        camera.id()
    };
    commands.entity(body_id).add_child(camera_id);

    // UI overlay camera
    if let Some(msaa) = app_config.msaa_component() {
        commands.spawn((
            Camera2d::default(),
            Camera {
                order: 1,
                clear_color: ClearColorConfig::Custom(Color::NONE),
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
                msaa_writeback: false,
                ..default()
            },
            msaa,
        ));
    } else {
        commands.spawn((
            Camera2d::default(),
            Camera {
                order: 1,
                clear_color: ClearColorConfig::Custom(Color::NONE),
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
                msaa_writeback: false,
                ..default()
            },
        ));
    }
}
