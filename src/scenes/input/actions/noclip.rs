use bevy::{
    input::keyboard::KeyCode,
    prelude::{ButtonInput, Commands, Entity, Query, Res, ResMut, Vec3, With},
};
use bevy_rapier3d::prelude::{GravityScale, RigidBody, Sensor, Velocity};

use super::super::types::{ActionStates, NoclipState, PlayerBody, SceneNoclipConfig};

pub fn apply_noclip_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneNoclipConfig>>,
    states: Option<Res<ActionStates>>,
    state: Option<ResMut<NoclipState>>,
    mut bodies: Query<
        (
            Entity,
            &mut RigidBody,
            &mut GravityScale,
            &mut Velocity,
            Option<&Sensor>,
        ),
        With<PlayerBody>,
    >,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut state) = state else {
        return;
    };

    if config.action.toggle && states.get(&config.id).just_pressed {
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
            *body = RigidBody::KinematicPositionBased;
            gravity.0 = 0.0;
            velocity.linear = Vec3::ZERO;
            velocity.angular = Vec3::ZERO;
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
