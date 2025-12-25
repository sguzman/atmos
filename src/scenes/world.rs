use bevy::prelude::*;
use serde::Deserialize;

use crate::scenes::config::{EntityOverrides, TransformOverrides};
use crate::scenes::config::{
    BoundingBoxConfig, CameraConfig, LightEntry, SkyboxConfig, SunConfig,
};

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    #[serde(default)]
    pub entities: Vec<EntityPlacement>,
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
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            camera: CameraConfig::default(),
            bounds: BoundingBoxConfig::default(),
            lights: Vec::new(),
            skybox: None,
            sun: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EntityPlacement {
    pub template: String,
    #[serde(default)]
    pub name_override: Option<String>,
    #[serde(default)]
    pub transform: TransformOverrides,
    #[serde(default)]
    pub overrides: EntityOverrides,
}
