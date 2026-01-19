use bevy::prelude::{
    Commands, GlobalTransform, Local,
    Query, Res, Time, Transform, Vec3,
    With,
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Ccd,
    Collider, Friction, Restitution,
    RigidBody, Velocity,
};

use crate::scenes::bounds::DespawnOutsideBounds;
use crate::scenes::spawn::SceneEntityTag;

use super::super::types::{
    ActionStates, SceneCamera,
    SceneShootConfig,
};

#[derive(Default)]
pub(crate) struct ShootState {
    accumulator: f32,
    delay_remaining: f32,
}

pub fn apply_shoot_action(
    time: Res<Time>,
    config: Option<
        Res<SceneShootConfig>,
    >,
    states: Option<Res<ActionStates>>,
    mut state: Local<ShootState>,
    cameras: Query<
        &GlobalTransform,
        With<SceneCamera>,
    >,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let action_state =
        states.get(&config.id);

    if !action_state.pressed {
        state.accumulator = 0.0;
        state.delay_remaining = 0.0;
        return;
    }

    let Ok(camera) = cameras.single()
    else {
        return;
    };

    let rate =
        config.action.rate.max(0.1);
    let interval = 1.0 / rate;
    let forward = camera.forward();
    let spin = Vec3::new(
        config
            .action
            .spin
            .x
            .to_radians(),
        config
            .action
            .spin
            .y
            .to_radians(),
        config
            .action
            .spin
            .z
            .to_radians(),
    );

    let spawn_ball =
        |commands: &mut Commands| {
            let spawn_pos = camera
                .translation()
                + forward
                    * config
                        .action
                        .spawn_offset;
            let mut entity = commands.spawn((
            bevy::prelude::Name::new(config.name.clone()),
            bevy::prelude::Mesh3d(config.mesh.clone()),
            bevy::prelude::MeshMaterial3d(config.material.clone()),
            Transform::from_translation(spawn_pos),
            SceneEntityTag,
            DespawnOutsideBounds,
            Velocity {
                linvel: forward * config.action.velocity,
                angvel: spin,
            },
            bevy::prelude::Visibility::default(),
            bevy::prelude::InheritedVisibility::default(),
            bevy::prelude::ViewVisibility::default(),
        ));

            if let Some(physics) =
                config.physics.as_ref()
            {
                if !physics.enabled {
                    return;
                }
                let rigid_body =
                    resolve_rigid_body(
                        &physics
                            .body_type,
                    );
                entity.insert((
                rigid_body,
                Collider::ball(config.shape.radius.unwrap_or(0.2)),
                Restitution::coefficient(physics.restitution),
                Friction::coefficient(physics.friction),
            ));
                if matches!(
                    rigid_body,
                    RigidBody::Dynamic
                ) && physics.mass
                    > 0.0
                {
                    entity.insert(AdditionalMassProperties::Mass(physics.mass));
                }
                if config.action.ccd {
                    entity.insert(
                        Ccd::enabled(),
                    );
                }
            }
        };

    let dt = time.delta_secs();
    if action_state.just_pressed {
        state.delay_remaining = config
            .action
            .start_delay
            .max(0.0);
        state.accumulator = 0.0;
        if state.delay_remaining <= 0.0
        {
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
    while state.accumulator >= interval
    {
        state.accumulator -= interval;
        spawn_ball(&mut commands);
    }
}

fn resolve_rigid_body(
    body_type: &str,
) -> RigidBody {
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
