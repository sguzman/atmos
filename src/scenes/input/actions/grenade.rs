use bevy::{
    prelude::{Commands, Component, GlobalTransform, Query, Res, Time, Transform, Vec3, With},
    time::Timer,
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, Friction, Restitution, RigidBody, Velocity,
};

use crate::scenes::bounds::DespawnOutsideBounds;
use crate::scenes::spawn::SceneEntityTag;

use super::super::types::{ActionStates, SceneCamera, SceneGrenadeConfig};

#[derive(Component)]
pub(crate) struct GrenadeFuse {
    timer: Timer,
    radius: f32,
    force: f32,
}

pub fn apply_grenade_action(
    config: Option<Res<SceneGrenadeConfig>>,
    states: Option<Res<ActionStates>>,
    cameras: Query<&GlobalTransform, With<SceneCamera>>,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    if !states.get(&config.id).just_pressed {
        return;
    }

    let Ok(camera) = cameras.single() else {
        return;
    };

    let forward = camera.forward();
    let spawn_pos = camera.translation() + forward * config.action.spawn_offset;
    let spin = Vec3::new(
        config.action.spin.x.to_radians(),
        config.action.spin.y.to_radians(),
        config.action.spin.z.to_radians(),
    );

    let mut entity = commands.spawn((
        bevy::prelude::Name::new(config.name.clone()),
        bevy::prelude::Mesh3d(config.mesh.clone()),
        bevy::prelude::MeshMaterial3d(config.material.clone()),
        Transform::from_translation(spawn_pos),
        SceneEntityTag,
        DespawnOutsideBounds,
        Velocity {
            linear: forward * config.action.velocity,
            angular: spin,
        },
        GrenadeFuse {
            timer: Timer::from_seconds(
                config.action.fuse_seconds.max(0.0),
                bevy::time::TimerMode::Once,
            ),
            radius: config.action.explosion_radius.max(0.0),
            force: config.action.explosion_force.max(0.0),
        },
        bevy::prelude::Visibility::default(),
        bevy::prelude::InheritedVisibility::default(),
        bevy::prelude::ViewVisibility::default(),
    ));

    if let Some(physics) = config.physics.as_ref() {
        if physics.enabled {
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
                entity.insert(bevy_rapier3d::prelude::Ccd::enabled());
            }
            return;
        }
    }

    entity.insert((
        RigidBody::Dynamic,
        Collider::ball(config.shape.radius.unwrap_or(0.2)),
    ));
    if config.action.ccd {
        entity.insert(bevy_rapier3d::prelude::Ccd::enabled());
    }
}

pub fn update_grenade_fuses(
    time: Res<Time>,
    mut grenades: Query<(bevy::prelude::Entity, &mut GrenadeFuse, &GlobalTransform)>,
    mut bodies: Query<(
        bevy::prelude::Entity,
        &GlobalTransform,
        &RigidBody,
        Option<&mut Velocity>,
    )>,
    mut commands: Commands,
) {
    let dt = time.delta();
    for (grenade_entity, mut fuse, transform) in grenades.iter_mut() {
        fuse.timer.tick(dt);
        if !fuse.timer.is_finished() {
            continue;
        }

        let origin = transform.translation();
        let radius = fuse.radius;
        if radius > 0.0 && fuse.force > 0.0 {
            for (entity, body_transform, body, velocity) in bodies.iter_mut() {
                if entity == grenade_entity {
                    continue;
                }
                if !matches!(*body, RigidBody::Dynamic) {
                    continue;
                }
                let delta = body_transform.translation() - origin;
                let distance = delta.length();
                if distance > radius || distance <= f32::EPSILON {
                    continue;
                }
                let strength = fuse.force * (1.0 - (distance / radius));
                let impulse = delta.normalize() * strength;
                if let Some(mut velocity) = velocity {
                    velocity.linear += impulse;
                } else {
                    commands.entity(entity).insert(Velocity {
                        linear: impulse,
                        angular: Vec3::ZERO,
                    });
                }
            }
        }

        commands
            .entity(grenade_entity)
            .queue_silenced(bevy::ecs::system::entity_command::despawn());
    }
}

fn resolve_rigid_body(body_type: &str) -> RigidBody {
    match body_type.trim().to_ascii_lowercase().as_str() {
        "fixed" | "static" => RigidBody::Fixed,
        "kinematic_position" | "kinematic_position_based" => RigidBody::KinematicPositionBased,
        "kinematic_velocity" | "kinematic_velocity_based" => RigidBody::KinematicVelocityBased,
        _ => RigidBody::Dynamic,
    }
}
