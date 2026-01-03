use bevy::{
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    prelude::{
        ButtonInput, Commands, Entity, GlobalTransform, Local, Projection, Query, Res, ResMut, Time, Transform, Vec3, With,
    },
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Ccd, Collider, Friction, GravityScale, QueryFilter, ReadRapierContext,
    Restitution, RigidBody, Sensor, Velocity,
};

use crate::scenes::bounds::DespawnOutsideBounds;
use crate::scenes::bounds::SceneBounds;

use super::types::{
    NoclipState, PlayerBody, PlayerSpawn, SceneCamera, SceneFovConfig, SceneJumpConfig,
    SceneNoclipConfig, SceneShootConfig, SceneSprintConfig, SceneZoomConfig, SprintState,
    ZoomState,
};

#[derive(Default)]
pub(crate) struct ShootState {
    accumulator: f32,
    delay_remaining: f32,
}

#[derive(Default)]
pub(crate) struct JumpState {
    cooldown_remaining: f32,
}

pub fn apply_sprint_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneSprintConfig>>,
    mut state: ResMut<SprintState>,
) {
    let Some(config) = config else {
        return;
    };
    if config.action.toggle && keys.just_pressed(config.trigger) {
        state.active = !state.active;
    }
}

pub fn apply_zoom_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneZoomConfig>>,
    mut state: ResMut<ZoomState>,
    mut cameras: Query<&mut Projection, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

    let Ok(mut projection) = cameras.single_mut() else {
        return;
    };

    if state.base_fov.is_none() {
        if let Projection::Perspective(ref perspective) = *projection {
            state.base_fov = Some(perspective.fov);
        }
    }

    let Some(base_fov) = state.base_fov else {
        return;
    };

    let was_active = state.active;
    if config.action.toggle {
        if keys.just_pressed(config.trigger) {
            state.active = !state.active;
        }
    } else {
        state.active = keys.pressed(config.trigger);
    }

    if !was_active && state.active {
        if let Projection::Perspective(ref perspective) = *projection {
            state.base_fov = Some(perspective.fov);
        }
    }

    if state.active {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = config.action.fov_degrees.to_radians();
        }
    } else if was_active {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = base_fov;
        }
    }
}

pub fn apply_noclip_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneNoclipConfig>>,
    state: Option<ResMut<NoclipState>>,
    mut bodies: Query<(Entity, &mut RigidBody, &mut GravityScale, &mut Velocity, Option<&Sensor>), With<PlayerBody>>,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };
    let Some(mut state) = state else {
        return;
    };

    if config.action.toggle && keys.just_pressed(config.trigger) {
        state.active = !state.active;
    }

    if let Ok((entity, mut body, mut gravity, mut velocity, sensor)) = bodies.single_mut() {
        if state.active {
            if config.action.speed_toggle {
                if let Some(key) = config.speed_toggle_key {
                    if keys.just_pressed(key) {
                        state.fast = !state.fast;
                    }
                }
            } else if let Some(key) = config.speed_toggle_key {
                state.fast = keys.pressed(key);
            }
            *body = RigidBody::KinematicVelocityBased;
            gravity.0 = 0.0;
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
            if sensor.is_none() {
                commands.entity(entity).insert(Sensor);
            }
            state.velocity = Vec3::ZERO;
        } else {
            *body = RigidBody::Dynamic;
            gravity.0 = 1.0;
            if sensor.is_some() {
                commands.entity(entity).remove::<Sensor>();
            }
            state.fast = false;
        }
    }
}

pub fn apply_fov_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneFovConfig>>,
    zoom_state: Option<ResMut<ZoomState>>,
    mut cameras: Query<&mut Projection, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

    let mut selected = None;
    for binding in &config.bindings {
        if keys.just_pressed(binding.trigger) {
            selected = Some(binding.fov_degrees);
        }
    }

    let Some(fov_degrees) = selected else {
        return;
    };

    let fov_radians = fov_degrees.to_radians();
    if let Some(mut zoom_state) = zoom_state {
        zoom_state.base_fov = Some(fov_radians);
        if zoom_state.active {
            return;
        }
    }

    for mut projection in cameras.iter_mut() {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = fov_radians;
        }
    }
}

pub fn apply_jump_action(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneJumpConfig>>,
    noclip: Option<Res<NoclipState>>,
    mut state: Local<JumpState>,
    rapier_context: ReadRapierContext,
    mut bodies: Query<(Entity, &Transform, &mut Velocity), With<PlayerBody>>,
) {
    let Some(config) = config else {
        return;
    };

    if state.cooldown_remaining > 0.0 {
        state.cooldown_remaining -= time.delta_secs();
    }

    if noclip.as_ref().is_some_and(|state| state.active) {
        return;
    }

    let Ok((entity, transform, mut velocity)) = bodies.single_mut() else {
        return;
    };

    if !keys.just_pressed(config.trigger) {
        return;
    }

    let Ok(context) = rapier_context.single() else {
        return;
    };

    let ray_origin = transform.translation;
    let ray_dir = Vec3::NEG_Y;
    let grounded = context
        .cast_ray(
            ray_origin,
            ray_dir,
            config.action.ground_check_distance.max(0.0),
            true,
            QueryFilter {
                exclude_rigid_body: Some(entity),
                ..Default::default()
            },
        )
        .is_some();

    if grounded {
        velocity.linvel.y = config.action.velocity.max(0.0);
        state.cooldown_remaining = config.action.cooldown.max(0.0);
    }
}

pub fn apply_shoot_action(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    config: Option<Res<SceneShootConfig>>,
    mut state: Local<ShootState>,
    cameras: Query<&GlobalTransform, With<SceneCamera>>,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };

    if !buttons.pressed(config.trigger) {
        state.accumulator = 0.0;
        state.delay_remaining = 0.0;
        return;
    }

    let Ok(camera) = cameras.single() else {
        return;
    };

    let rate = config.action.rate.max(0.1);
    let interval = 1.0 / rate;
    let forward = camera.forward();
    let spin = Vec3::new(
        config.action.spin.x.to_radians(),
        config.action.spin.y.to_radians(),
        config.action.spin.z.to_radians(),
    );

    let spawn_ball = |commands: &mut Commands| {
        let spawn_pos = camera.translation() + forward * config.action.spawn_offset;
        let mut entity = commands.spawn((
            bevy::prelude::Name::new(config.name.clone()),
            bevy::prelude::Mesh3d(config.mesh.clone()),
            bevy::prelude::MeshMaterial3d(config.material.clone()),
            Transform::from_translation(spawn_pos),
            DespawnOutsideBounds,
            Velocity {
                linvel: forward * config.action.velocity,
                angvel: spin,
            },
            bevy::prelude::Visibility::default(),
            bevy::prelude::InheritedVisibility::default(),
            bevy::prelude::ViewVisibility::default(),
        ));

        if let Some(physics) = config.physics.as_ref() {
            if !physics.enabled {
                return;
            }
            let rigid_body = resolve_rigid_body(&physics.body_type);
            entity.insert((
                rigid_body,
                Collider::ball(config.shape.radius.unwrap_or(0.2)),
                Restitution::coefficient(physics.restitution),
                Friction::coefficient(physics.friction),
            ));
            if matches!(rigid_body, RigidBody::Dynamic) && physics.mass > 0.0 {
                entity.insert(AdditionalMassProperties::Mass(physics.mass));
            }
            if config.action.ccd {
                entity.insert(Ccd::enabled());
            }
        }
    };

    let dt = time.delta_secs();
    if buttons.just_pressed(config.trigger) {
        state.delay_remaining = config.action.start_delay.max(0.0);
        state.accumulator = 0.0;
        if state.delay_remaining <= 0.0 {
            spawn_ball(&mut commands);
        }
    }

    if state.delay_remaining > 0.0 {
        state.delay_remaining -= dt;
        if state.delay_remaining > 0.0 {
            return;
        }
        spawn_ball(&mut commands);
        state.accumulator = 0.0;
    }

    state.accumulator += dt;
    while state.accumulator >= interval {
        state.accumulator -= interval;
        spawn_ball(&mut commands);
    }
}

pub fn apply_player_respawn(
    bounds: Option<Res<SceneBounds>>,
    spawn: Option<Res<PlayerSpawn>>,
    mut bodies: Query<(&mut Transform, &mut Velocity), With<PlayerBody>>,
    noclip: Option<ResMut<NoclipState>>,
) {
    let (Some(bounds), Some(spawn)) = (bounds, spawn) else {
        return;
    };

    let Ok((mut transform, mut velocity)) = bodies.single_mut() else {
        return;
    };

    if transform.translation.y >= bounds.min.y {
        return;
    }

    transform.translation = spawn.position;
    velocity.linvel = Vec3::ZERO;
    velocity.angvel = Vec3::ZERO;
    if let Some(mut noclip) = noclip {
        noclip.velocity = Vec3::ZERO;
    }
}

fn resolve_rigid_body(body_type: &str) -> RigidBody {
    match body_type.trim().to_ascii_lowercase().as_str() {
        "fixed" | "static" => RigidBody::Fixed,
        "kinematic_position" | "kinematic_position_based" => {
            RigidBody::KinematicPositionBased
        }
        "kinematic_velocity" | "kinematic_velocity_based" => {
            RigidBody::KinematicVelocityBased
        }
        _ => RigidBody::Dynamic,
    }
}
