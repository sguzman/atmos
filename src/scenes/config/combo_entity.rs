use serde::Deserialize;

use super::{EntityOverrides, TransformOverrides, Vec3Config};

#[derive(Debug, Deserialize, Clone)]
pub struct ComboTemplate {
    pub name: String,
    #[serde(default)]
    pub parts: Vec<ComboPart>,
    #[serde(default)]
    pub overrides: Option<EntityOverrides>,
    #[serde(default)]
    pub physics: Option<ComboPhysics>,
    #[serde(default)]
    pub stack: Option<ComboStackConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ComboPart {
    pub template: String,
    #[serde(default)]
    pub name_override: Option<String>,
    #[serde(default)]
    pub transform: Option<TransformOverrides>,
    #[serde(default)]
    pub overrides: Option<EntityOverrides>,
    #[serde(default)]
    pub attach: Option<AttachConfig>,
    #[serde(default)]
    pub physics_root: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ComboPhysics {
    #[serde(default)]
    pub shared: bool,
    #[serde(default)]
    pub root: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AttachConfig {
    pub target: String,
    #[serde(default)]
    pub offset: Vec3Config,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ComboStackConfig {
    pub count: u32,
    #[serde(default)]
    pub spacing: Vec3Config,
    #[serde(default)]
    pub start_offset: Vec3Config,
}
