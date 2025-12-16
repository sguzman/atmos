use std::fs;

use crate::scenes::config::{
    camera_config_path, circle_config_path, cube_config_path, input_config_path, light_config_path,
    CameraConfig, CircleConfig, CubeConfig, InputConfig, LightConfig,
};
use crate::scenes::world::WorldConfig;
use bevy::log::{info, warn};

pub fn load_cube_config(scene: &str) -> CubeConfig {
    let path = cube_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return CubeConfig::default();
        }
    };

    match toml::from_str::<CubeConfig>(&contents) {
        Ok(config) => {
            info!("Loaded cube config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            CubeConfig::default()
        }
    }
}

pub fn load_circle_config(scene: &str) -> CircleConfig {
    let path = circle_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return CircleConfig::default();
        }
    };

    match toml::from_str::<CircleConfig>(&contents) {
        Ok(config) => {
            info!("Loaded circle config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            CircleConfig::default()
        }
    }
}

pub fn load_camera_config(scene: &str) -> CameraConfig {
    let path = camera_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return CameraConfig::default();
        }
    };

    match toml::from_str::<CameraConfig>(&contents) {
        Ok(config) => {
            info!("Loaded camera config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            CameraConfig::default()
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

pub fn load_light_config(scene: &str) -> LightConfig {
    let path = light_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return LightConfig::default();
        }
    };

    match toml::from_str::<LightConfig>(&contents) {
        Ok(config) => {
            info!("Loaded light config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            LightConfig::default()
        }
    }
}
