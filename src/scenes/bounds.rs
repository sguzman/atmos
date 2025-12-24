use bevy::prelude::*;

use crate::scenes::config::BoundingBoxConfig;

#[derive(Resource, Clone)]
pub struct SceneBounds {
    pub shape: String,
    pub min: Vec3,
    pub max: Vec3,
}

impl From<BoundingBoxConfig> for SceneBounds {
    fn from(config: BoundingBoxConfig) -> Self {
        Self {
            shape: config.shape,
            min: Vec3::new(config.x.min, config.y.min, config.z.min),
            max: Vec3::new(config.x.max, config.y.max, config.z.max),
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
    for (entity, transform) in &query {
        let pos = transform.translation;
        if pos.x < bounds.min.x
            || pos.x > bounds.max.x
            || pos.y < bounds.min.y
            || pos.y > bounds.max.y
            || pos.z < bounds.min.z
            || pos.z > bounds.max.z
        {
            commands.entity(entity).despawn();
        }
    }
}
