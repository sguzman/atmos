use bevy::{
    input::keyboard::KeyCode,
    prelude::{ButtonInput, Entity, Local, Query, Res, Time, Transform, Vec3, With},
};
use bevy_rapier3d::prelude::{QueryFilter, ReadRapierContext, Velocity};

use super::super::types::{NoclipState, PlayerBody, SceneJumpConfig};

#[derive(Default)]
pub(crate) struct JumpState {
    cooldown_remaining: f32,
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
