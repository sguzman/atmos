use serde::Deserialize;

use crate::scenes::config::{EntityOverrides, TransformOverrides};

#[derive(Debug, Deserialize, Default)]
pub struct EntitiesConfig {
    #[serde(default)]
    pub entities: Vec<EntityPlacement>,
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
