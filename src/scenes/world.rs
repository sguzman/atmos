use bevy::prelude::*;
use serde::Deserialize;

use crate::scenes::config::{
    BoundingBoxConfig, CameraConfig, LightEntry, RenderConfig, SkyboxConfig, SunConfig,
};

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    #[serde(default)]
    pub camera: CameraConfig,
    #[serde(default)]
    pub bounds: BoundingBoxConfig,
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
            lights: Vec::new(),
            skybox: None,
            sun: None,
            render: None,
        }
    }
}
