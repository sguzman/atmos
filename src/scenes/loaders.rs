use std::collections::{HashMap, HashSet};

use crate::scenes::config::{
    action_config_path, actions_config_path, ActionsConfig, ComboTemplate, DialogueConfig,
    EntityTemplate, InputConfig, OverlayConfig, dialogue_config_path, input_config_path,
    overlay_config_path,
};
use crate::scenes::entities::EntitiesConfig;
use crate::scenes::world::WorldConfig;
use crate::scenes::TomlAsset;
use bevy::asset::LoadState;
use bevy::log::{info, warn};
use bevy::prelude::{AssetServer, Assets, Handle, Resource};
use serde::de::DeserializeOwned;

#[derive(Resource, Default)]
pub struct TomlCache {
    handles: HashMap<String, Handle<TomlAsset>>,
    warned_missing: HashSet<String>,
    warned_parse: HashSet<String>,
    loaded: HashSet<String>,
}

pub enum ConfigLoad<T> {
    Pending,
    Ready(T),
}

impl TomlCache {
    fn handle_for(&mut self, asset_server: &AssetServer, path: &str) -> Handle<TomlAsset> {
        if let Some(handle) = self.handles.get(path) {
            return handle.clone();
        }
        let handle = asset_server.load(path.to_string());
        self.handles.insert(path.to_string(), handle.clone());
        handle
    }

    fn warn_missing_once(&mut self, path: &str, err: &str) {
        if self.warned_missing.insert(path.to_string()) {
            warn!("Failed to load {path}: {err}");
        }
    }

    fn warn_parse_once(&mut self, path: &str, err: &str) {
        if self.warned_parse.insert(path.to_string()) {
            warn!("Failed to parse {path}: {err}");
        }
    }
}

fn load_toml_config<T>(
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
    path: &str,
    default: T,
    label: &str,
) -> ConfigLoad<T>
where
    T: DeserializeOwned,
{
    let handle = cache.handle_for(asset_server, path);
    match asset_server.get_load_state(handle.id()) {
        Some(LoadState::Loaded) => {
            let Some(asset) = toml_assets.get(&handle) else {
                return ConfigLoad::Pending;
            };
            match toml::from_str::<T>(&asset.0) {
                Ok(config) => {
                    if cache.loaded.insert(path.to_string()) {
                        info!("Loaded {label} config from {path}.");
                    }
                    ConfigLoad::Ready(config)
                }
                Err(err) => {
                    cache.warn_parse_once(path, &err.to_string());
                    ConfigLoad::Ready(default)
                }
            }
        }
        Some(LoadState::Failed(err)) => {
            cache.warn_missing_once(path, &err.to_string());
            ConfigLoad::Ready(default)
        }
        _ => ConfigLoad::Pending,
    }
}

pub fn load_entity_template_from_path(
    scene: &str,
    template_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<EntityTemplate>> {
    let path = action_config_path(scene, template_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "entity template",
    )
}

pub fn load_combo_template_from_path(
    scene: &str,
    template_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<ComboTemplate>> {
    let path = action_config_path(scene, template_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "combo template",
    )
}

pub fn load_input_config(
    scene: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<InputConfig> {
    let path = input_config_path(scene);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        InputConfig::default(),
        "input",
    )
}

pub fn load_actions_config(
    scene: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<ActionsConfig> {
    let path = actions_config_path(scene);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        ActionsConfig::default(),
        "actions",
    )
}

pub fn load_world_config(
    scene: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<WorldConfig> {
    let path = format!("{root}/{scene}/world.toml", root = crate::scenes::config::SCENE_ROOT);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        WorldConfig::default(),
        "world",
    )
}

pub fn load_entities_config(
    scene: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<EntitiesConfig> {
    let path = format!("{root}/{scene}/entities.toml", root = crate::scenes::config::SCENE_ROOT);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        EntitiesConfig::default(),
        "entities",
    )
}

pub fn load_overlay_config(
    name: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<OverlayConfig> {
    let path = overlay_config_path(name);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        OverlayConfig::default(),
        "overlay",
    )
}

pub fn load_dialogue_config(
    name: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<DialogueConfig> {
    let path = dialogue_config_path(name);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        DialogueConfig {
            start: String::new(),
            nodes: Vec::new(),
        },
        "dialogue",
    )
}
