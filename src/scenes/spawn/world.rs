use bevy::{
    log::warn,
    prelude::*,
};

use crate::scenes::{
    config::ActiveScene,
    entities::EntitiesConfig,
    loaders::{load_combo_template_from_path, load_entity_template_from_path},
};

use super::combo::spawn_combo_template;
use super::entities::spawn_entity_from_template;

pub(super) fn spawn_world_entities(
    entities: &EntitiesConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) {
    for entity in &entities.entities {
        if entity.template.starts_with("combo/")
            || entity.template.ends_with(".combo.toml")
        {
            let Some(combo) =
                load_combo_template_from_path(&active_scene.name, &entity.template)
            else {
                warn!(
                    "Failed to load combo template '{}' in scene '{}'; skipping.",
                    entity.template, active_scene.name
                );
                continue;
            };
            spawn_combo_template(
                &combo,
                &entity.transform,
                &entity.overrides,
                entity.name_override.as_ref(),
                commands,
                meshes,
                materials,
                asset_server,
                active_scene,
            );
        } else {
            let Some(template) =
                load_entity_template_from_path(&active_scene.name, &entity.template)
            else {
                warn!(
                    "Failed to load template '{}' in scene '{}'; skipping.",
                    entity.template, active_scene.name
                );
                continue;
            };
            spawn_entity_from_template(
                &template,
                &entity.overrides,
                &entity.transform,
                entity.name_override.as_ref(),
                commands,
                meshes,
                materials,
                asset_server,
                active_scene,
            );
        }
    }
}
