use bevy::{
    log::warn,
    prelude::*,
};

use crate::scenes::{
    config::{
        ActiveScene, CircleConfig, CubeConfig, LightConfig, PillarComboConfig, RectangleConfig,
    },
    world::WorldConfig,
};

use super::combos::spawn_pillar_with_light;
use super::lights::spawn_light_entity;
use super::shapes::{spawn_circle, spawn_cube, spawn_rectangle};

pub(super) fn spawn_world_entities(
    world: &WorldConfig,
    circle_template: &CircleConfig,
    cube_template: &CubeConfig,
    rectangle_template: &RectangleConfig,
    top_light_template: &LightConfig,
    combo_template: &PillarComboConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    for entity in &world.entities {
        match entity.template.as_str() {
            path if path.ends_with("circle.toml") => spawn_circle(
                entity,
                circle_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            path if path.ends_with("cube.toml") => spawn_cube(
                entity,
                cube_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            path if path.ends_with("rectangle.toml") => spawn_rectangle(
                entity,
                rectangle_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            path if path.ends_with("top_light.toml") => spawn_light_entity(
                entity,
                top_light_template,
                commands,
                active_scene,
            ),
            path if path.ends_with("pillar_with_light.toml") => spawn_pillar_with_light(
                entity,
                rectangle_template,
                top_light_template,
                combo_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            other => {
                warn!(
                    "Unknown template '{other}' in world; skipping entity placement in scene '{}'.",
                    active_scene.name
                );
            }
        }
    }
}
