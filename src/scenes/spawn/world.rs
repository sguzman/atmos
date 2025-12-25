use bevy::{
    log::warn,
    prelude::*,
};

use crate::scenes::{
    config::{
        ActiveScene, CircleConfig, CubeConfig, LightConfig, PillarComboConfig, RectangleConfig,
    },
    loaders::load_cube_config_from_path,
    world::WorldConfig,
};

use super::combos::spawn_pillar_with_light;
use super::lights::spawn_light_entity;
use super::shapes::{spawn_circle, spawn_cube, spawn_rectangle};
use super::stack::spawn_rectangle_stack;
use crate::scenes::loaders::load_rectangle_stack_config;

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
            path if path.ends_with("cube.toml") => {
                if path == "entities/cube.toml" {
                    spawn_cube(
                        entity,
                        cube_template,
                        commands,
                        meshes,
                        materials,
                        active_scene,
                    );
                } else if let Some(config) =
                    load_cube_config_from_path(&active_scene.name, path)
                {
                    spawn_cube(
                        entity,
                        &config,
                        commands,
                        meshes,
                        materials,
                        active_scene,
                    );
                }
            }
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
            path if path.ends_with("stack_of_rectangles.toml") => {
                if let Some(stack_config) =
                    load_rectangle_stack_config(&active_scene.name, path)
                {
                    spawn_rectangle_stack(
                        entity,
                        rectangle_template,
                        &stack_config,
                        commands,
                        meshes,
                        materials,
                        active_scene,
                    );
                }
            }
            other => {
                warn!(
                    "Unknown template '{other}' in world; skipping entity placement in scene '{}'.",
                    active_scene.name
                );
            }
        }
    }
}
