use bevy::prelude::*;
use serde::Deserialize;

use crate::scenes::config::{
    CubeRotationConfig, LightOverrides, PositionConfig, RectangleOverrides,
};

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    #[serde(default)]
    pub entities: Vec<EntityPlacement>,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EntityPlacement {
    pub template: String,
    #[serde(default)]
    pub name_override: Option<String>,
    #[serde(default)]
    pub transform: EntityTransform,
    #[serde(default)]
    pub radius: Option<f32>, // used by circle templates
    #[serde(default)]
    pub rectangle: Option<RectangleOverrides>,
    #[serde(default)]
    pub light: Option<LightOverrides>,
}

#[derive(Debug, Deserialize)]
pub struct EntityTransform {
    #[serde(default)]
    pub position: PositionConfig,
    #[serde(default = "default_unit_scale")]
    pub scale: f32,
    #[serde(default)]
    pub rotation: CubeRotationConfig,
}

impl Default for EntityTransform {
    fn default() -> Self {
        Self {
            position: PositionConfig::default(),
            scale: default_unit_scale(),
            rotation: CubeRotationConfig::default(),
        }
    }
}

fn default_unit_scale() -> f32 {
    1.0
}
