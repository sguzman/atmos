use std::collections::{HashMap, HashSet};

use crate::scenes::config::{
    action_config_path, ComboTemplate, EntityTemplate, GrabActionConfig, InputConfig,
    JumpActionConfig, NoclipActionConfig, OverlayConfig, QuitActionConfig,
    SceneTransitionActionConfig, ShootActionConfig, SprintActionConfig, ZoomActionConfig,
    input_config_path, overlay_config_path,
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
                    info!("Loaded {label} config from {path}.");
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

pub fn load_shoot_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<ShootActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "shoot action",
    )
}

pub fn load_sprint_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<SprintActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "sprint action",
    )
}

pub fn load_zoom_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<ZoomActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "zoom action",
    )
}

pub fn load_jump_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<JumpActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "jump action",
    )
}

pub fn load_noclip_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<NoclipActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "noclip action",
    )
}

pub fn load_grab_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<GrabActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "grab action",
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

pub fn load_scene_transition_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<SceneTransitionActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "scene transition action",
    )
}

pub fn load_quit_action_config(
    scene: &str,
    action_path: &str,
    cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> ConfigLoad<Option<QuitActionConfig>> {
    let path = action_config_path(scene, action_path);
    load_toml_config(
        cache,
        asset_server,
        toml_assets,
        &path,
        None,
        "quit action",
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
