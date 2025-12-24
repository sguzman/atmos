use bevy::prelude::*;

use crate::scenes::config::BoundingBoxConfig;

#[derive(Resource, Clone)]
pub struct SceneBounds {
    pub shape: String,
    pub half_extents: Vec3,
}

impl From<BoundingBoxConfig> for SceneBounds {
    fn from(config: BoundingBoxConfig) -> Self {
        let half_extents = Vec3::new(
            config.dimensions.width * 0.5,
            config.dimensions.height * 0.5,
            config.dimensions.depth * 0.5,
        );
        Self {
            shape: config.shape,
            half_extents,
        }
    }
}

#[derive(Component)]
pub struct DespawnOutsideBounds;

pub fn despawn_out_of_bounds(
    bounds: Option<Res<SceneBounds>>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<DespawnOutsideBounds>>,
) {
    let Some(bounds) = bounds else {
        return;
    };
    if bounds.shape.trim().to_ascii_lowercase() != "rectangle" {
        return;
    }
    let half = bounds.half_extents;
    for (entity, transform) in &query {
        let pos = transform.translation;
        if pos.x.abs() > half.x || pos.y.abs() > half.y || pos.z.abs() > half.z {
            commands.entity(entity).despawn();
        }
    }
}
