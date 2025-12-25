use bevy::prelude::*;
use serde::Deserialize;

use crate::scenes::config::{EntityOverrides, TransformOverrides};

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
    pub transform: TransformOverrides,
    #[serde(default)]
    pub overrides: EntityOverrides,
}
