use bevy::prelude::*;
use serde::Deserialize;

use crate::scenes::config::{
    BoundingBoxConfig, CameraConfig, LightEntry, RenderConfig, SkyboxConfig, SunConfig, Vec3Config,
};

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    #[serde(default)]
    pub camera: CameraConfig,
    #[serde(default)]
    pub bounds: BoundingBoxConfig,
    #[serde(default)]
    pub gravity: Option<Vec3Config>,
    #[serde(default)]
    pub lights: Vec<LightEntry>,
    #[serde(default)]
    pub skybox: Option<SkyboxConfig>,
    #[serde(default)]
    pub sun: Option<SunConfig>,
    #[serde(default)]
    pub render: Option<RenderConfig>,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            camera: CameraConfig::default(),
            bounds: BoundingBoxConfig::default(),
            gravity: None,
            lights: Vec::new(),
            skybox: None,
            sun: None,
            render: None,
        }
    }
}
