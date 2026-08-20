use bevy::prelude::{Query, Res, ResMut, Transform, Vec3, With};
use bevy_rapier3d::prelude::Velocity;

use crate::scenes::bounds::SceneBounds;

use super::super::types::{NoclipState, PlayerBody, PlayerSpawn};

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
