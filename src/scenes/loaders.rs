use std::fs;

use crate::scenes::config::{
    action_config_path, ComboTemplate, EntityTemplate, InputConfig, OverlayConfig,
    ShootActionConfig, SprintActionConfig, ZoomActionConfig,
    input_config_path, overlay_config_path,
};
use crate::scenes::entities::EntitiesConfig;
use crate::scenes::world::WorldConfig;
use bevy::log::{info, warn};

pub fn load_entity_template_from_path(scene: &str, template_path: &str) -> Option<EntityTemplate> {
    let path = action_config_path(scene, template_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Skipping entity template.");
            return None;
        }
    };

    match toml::from_str::<EntityTemplate>(&contents) {
        Ok(config) => {
            info!("Loaded entity template from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Skipping entity template.");
            None
        }
    }
}

pub fn load_combo_template_from_path(scene: &str, template_path: &str) -> Option<ComboTemplate> {
    let path = action_config_path(scene, template_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Skipping combo template.");
            return None;
        }
    };

    match toml::from_str::<ComboTemplate>(&contents) {
        Ok(config) => {
            info!("Loaded combo template from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Skipping combo template.");
            None
        }
    }
}

pub fn load_shoot_action_config(scene: &str, action_path: &str) -> Option<ShootActionConfig> {
    let path = action_config_path(scene, action_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Action disabled.");
            return None;
        }
    };

    match toml::from_str::<ShootActionConfig>(&contents) {
        Ok(config) => {
            info!("Loaded shoot action config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Action disabled.");
            None
        }
    }
}

pub fn load_sprint_action_config(scene: &str, action_path: &str) -> Option<SprintActionConfig> {
    let path = action_config_path(scene, action_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Action disabled.");
            return None;
        }
    };

    match toml::from_str::<SprintActionConfig>(&contents) {
        Ok(config) => {
            info!("Loaded sprint action config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Action disabled.");
            None
        }
    }
}

pub fn load_zoom_action_config(scene: &str, action_path: &str) -> Option<ZoomActionConfig> {
    let path = action_config_path(scene, action_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Action disabled.");
            return None;
        }
    };

    match toml::from_str::<ZoomActionConfig>(&contents) {
        Ok(config) => {
            info!("Loaded zoom action config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Action disabled.");
            None
        }
    }
}

pub fn load_input_config(scene: &str) -> InputConfig {
    let path = input_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return InputConfig::default();
        }
    };

    match toml::from_str::<InputConfig>(&contents) {
        Ok(config) => {
            info!("Loaded input config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            InputConfig::default()
        }
    }
}

pub fn load_world_config(scene: &str) -> WorldConfig {
    let path = format!("{root}/{scene}/world.toml", root = crate::scenes::config::SCENE_ROOT);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to empty world.");
            return WorldConfig::default();
        }
    };

    match toml::from_str::<WorldConfig>(&contents) {
        Ok(config) => {
            info!("Loaded world config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to empty world.");
            WorldConfig::default()
        }
    }
}

pub fn load_entities_config(scene: &str) -> EntitiesConfig {
    let path = format!("{root}/{scene}/entities.toml", root = crate::scenes::config::SCENE_ROOT);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to empty entities.");
            return EntitiesConfig::default();
        }
    };

    match toml::from_str::<EntitiesConfig>(&contents) {
        Ok(config) => {
            info!("Loaded entities config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to empty entities.");
            EntitiesConfig::default()
        }
    }
}

pub fn load_overlay_config(name: &str) -> OverlayConfig {
    let path = overlay_config_path(name);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to empty overlay.");
            return OverlayConfig::default();
        }
    };

    match toml::from_str::<OverlayConfig>(&contents) {
        Ok(config) => {
            info!("Loaded overlay config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to empty overlay.");
            OverlayConfig::default()
        }
    }
}
