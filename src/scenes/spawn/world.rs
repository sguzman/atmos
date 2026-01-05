use bevy::{
    log::warn,
    prelude::*,
};

use crate::scenes::{
    config::{ActiveScene, ComboTemplate, EntityTemplate},
    entities::EntitiesConfig,
    loaders::{load_combo_template_from_path, load_entity_template_from_path, ConfigLoad, TomlCache},
    MeshCacheSettings, TomlAsset,
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
    mesh_cache: &MeshCacheSettings,
    toml_assets: &Assets<TomlAsset>,
    toml_cache: &mut TomlCache,
) -> ConfigLoad<()> {
    let mut pending = false;
    let mut resolved = Vec::new();

    for entity in &entities.entities {
        let is_combo = entity.template.starts_with("combo/")
            || entity.template.ends_with(".combo.toml");
        if is_combo {
            match load_combo_template_from_path(
                &active_scene.name,
                &entity.template,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => pending = true,
                ConfigLoad::Ready(Some(combo)) => {
                    resolved.push((entity, ResolvedTemplate::Combo(combo)));
                }
                ConfigLoad::Ready(None) => {
                    warn!(
                        "Failed to load combo template '{}' in scene '{}'; skipping.",
                        entity.template, active_scene.name
                    );
                }
            }
        } else {
            match load_entity_template_from_path(
                &active_scene.name,
                &entity.template,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => pending = true,
                ConfigLoad::Ready(Some(template)) => {
                    resolved.push((entity, ResolvedTemplate::Entity(template)));
                }
                ConfigLoad::Ready(None) => {
                    warn!(
                        "Failed to load template '{}' in scene '{}'; skipping.",
                        entity.template, active_scene.name
                    );
                }
            }
        }
    }

    if pending {
        return ConfigLoad::Pending;
    }

    if combos_need_assets(&resolved, active_scene, toml_assets, asset_server, toml_cache) {
        return ConfigLoad::Pending;
    }

    for (entity, template) in resolved {
        match template {
            ResolvedTemplate::Combo(combo) => {
                let ready = spawn_combo_template(
                    &combo,
                    &entity.transform,
                    &entity.overrides,
                    entity.name_override.as_ref(),
                    mesh_cache,
                    commands,
                    meshes,
                    materials,
                    asset_server,
                    active_scene,
                    toml_assets,
                    toml_cache,
                );
                if matches!(ready, ConfigLoad::Pending) {
                    return ConfigLoad::Pending;
                }
            }
            ResolvedTemplate::Entity(template) => {
                spawn_entity_from_template(
                    &template,
                    &entity.overrides,
                    &entity.transform,
                    entity.name_override.as_ref(),
                    mesh_cache,
                    commands,
                    meshes,
                    materials,
                    asset_server,
                    active_scene,
                );
            }
        }
    }

    ConfigLoad::Ready(())
}

enum ResolvedTemplate {
    Combo(ComboTemplate),
    Entity(EntityTemplate),
}

fn combos_need_assets(
    resolved: &[( &crate::scenes::entities::EntityPlacement, ResolvedTemplate )],
    active_scene: &ActiveScene,
    toml_assets: &Assets<TomlAsset>,
    asset_server: &AssetServer,
    toml_cache: &mut TomlCache,
) -> bool {
    for (_entity, template) in resolved {
        let ResolvedTemplate::Combo(combo) = template else {
            continue;
        };
        for part in &combo.parts {
            match load_entity_template_from_path(
                &active_scene.name,
                &part.template,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return true,
                ConfigLoad::Ready(_) => {}
            }
        }
    }
    false
}
