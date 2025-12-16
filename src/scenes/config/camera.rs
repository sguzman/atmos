use serde::Deserialize;

use super::transforms::Vec3Config;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub name: String,
    #[serde(default)]
    pub transform: TransformConfig,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            name: "main_camera".to_string(),
            transform: TransformConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransformConfig {
    #[serde(default = "default_camera_position")]
    pub position: Vec3Config,
    #[serde(default = "default_camera_look_at")]
    pub look_at: Vec3Config,
    #[serde(default = "default_camera_up")]
    pub up: Vec3Config,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            position: default_camera_position(),
            look_at: default_camera_look_at(),
            up: default_camera_up(),
        }
    }
}

fn default_camera_position() -> Vec3Config {
    Vec3Config {
        x: -2.5,
        y: 4.5,
        z: 9.0,
    }
}

fn default_camera_look_at() -> Vec3Config {
    Vec3Config {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn default_camera_up() -> Vec3Config {
    Vec3Config {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }
}
