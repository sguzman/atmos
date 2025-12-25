use std::fs;

use crate::scenes::config::{
    action_config_path, bounding_box_config_path, camera_config_path, circle_config_path,
    cube_config_path, input_config_path, light_config_path, overlay_config_path,
    pillar_combo_config_path, rectangle_config_path, skybox_config_path, sphere_config_path,
    sun_config_path, top_light_config_path, BoundingBoxConfig, CameraConfig, CircleConfig,
    CubeConfig, InputConfig, LightConfig, OverlayConfig, PillarComboConfig, RectangleConfig,
    FovActionConfig, RectangleStackConfig, ShootActionConfig, SkyboxConfig, SphereConfig,
    SprintActionConfig, SunConfig,
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

pub fn load_cube_config_from_path(scene: &str, template_path: &str) -> Option<CubeConfig> {
    let path = action_config_path(scene, template_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Skipping cube template.");
            return None;
        }
    };

    match toml::from_str::<CubeConfig>(&contents) {
        Ok(config) => {
            info!("Loaded cube config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Skipping cube template.");
            None
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

pub fn load_rectangle_config(scene: &str) -> RectangleConfig {
    let path = rectangle_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return RectangleConfig::default();
        }
    };

    match toml::from_str::<RectangleConfig>(&contents) {
        Ok(config) => {
            info!("Loaded rectangle config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            RectangleConfig::default()
        }
    }
}

pub fn load_sphere_config(scene: &str) -> SphereConfig {
    let path = sphere_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return SphereConfig::default();
        }
    };

    match toml::from_str::<SphereConfig>(&contents) {
        Ok(config) => {
            info!("Loaded sphere config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            SphereConfig::default()
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

pub fn load_fov_action_config(scene: &str, action_path: &str) -> Option<FovActionConfig> {
    let path = action_config_path(scene, action_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Action disabled.");
            return None;
        }
    };

    match toml::from_str::<FovActionConfig>(&contents) {
        Ok(config) => {
            info!("Loaded fov action config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Action disabled.");
            None
        }
    }
}

pub fn load_rectangle_stack_config(
    scene: &str,
    stack_path: &str,
) -> Option<RectangleStackConfig> {
    let path = action_config_path(scene, stack_path);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Stack disabled.");
            return None;
        }
    };

    match toml::from_str::<RectangleStackConfig>(&contents) {
        Ok(config) => {
            info!("Loaded rectangle stack config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Stack disabled.");
            None
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

pub fn load_bounding_box_config(scene: &str) -> BoundingBoxConfig {
    let path = bounding_box_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return BoundingBoxConfig::default();
        }
    };

    match toml::from_str::<BoundingBoxConfig>(&contents) {
        Ok(config) => {
            info!("Loaded bounding box config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            BoundingBoxConfig::default()
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

pub fn load_sun_config(scene: &str) -> Option<SunConfig> {
    let path = sun_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. No sun will be spawned.");
            return None;
        }
    };

    match toml::from_str::<SunConfig>(&contents) {
        Ok(config) => {
            info!("Loaded sun config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. No sun will be spawned.");
            None
        }
    }
}

pub fn load_top_light_config(scene: &str) -> LightConfig {
    let path = top_light_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return LightConfig::default();
        }
    };

    match toml::from_str::<LightConfig>(&contents) {
        Ok(config) => {
            info!("Loaded top light config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            LightConfig::default()
        }
    }
}

pub fn load_skybox_config(scene: &str) -> Option<SkyboxConfig> {
    let path = skybox_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. No skybox will be applied.");
            return None;
        }
    };

    match toml::from_str::<SkyboxConfig>(&contents) {
        Ok(config) => {
            info!("Loaded skybox config from {path}.");
            Some(config)
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. No skybox will be applied.");
            None
        }
    }
}

pub fn load_pillar_combo_config(scene: &str) -> PillarComboConfig {
    let path = pillar_combo_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return PillarComboConfig::default();
        }
    };

    match toml::from_str::<PillarComboConfig>(&contents) {
        Ok(config) => {
            info!("Loaded combo config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            PillarComboConfig::default()
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
