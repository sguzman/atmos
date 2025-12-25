use serde::Deserialize;

use super::light::LightKind;
use super::physics::PhysicsConfig;
use super::transforms::{CubeRotationConfig, DimensionsConfig, PositionConfig, Vec3Config};

#[derive(Debug, Deserialize, Clone)]
pub struct EntityTemplate {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub transform: TransformConfig,
    #[serde(default)]
    pub shape: Option<ShapeConfig>,
    #[serde(default)]
    pub physics: Option<PhysicsConfig>,
    #[serde(default)]
    pub light: Option<LightComponent>,
    #[serde(default)]
    pub stack: Option<StackConfig>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct EntityOverrides {
    #[serde(default)]
    pub transform: Option<TransformOverrides>,
    #[serde(default)]
    pub shape: Option<ShapeOverrides>,
    #[serde(default)]
    pub physics: Option<PhysicsOverrides>,
    #[serde(default)]
    pub light: Option<LightOverridesConfig>,
    #[serde(default)]
    pub stack: Option<StackOverrides>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TransformConfig {
    #[serde(default)]
    pub position: PositionConfig,
    #[serde(default)]
    pub rotation: CubeRotationConfig,
    #[serde(default = "default_unit_scale")]
    pub scale: f32,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            position: PositionConfig::default(),
            rotation: CubeRotationConfig::default(),
            scale: default_unit_scale(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TransformOverrides {
    #[serde(default)]
    pub position: Option<PositionConfig>,
    #[serde(default)]
    pub rotation: Option<CubeRotationConfig>,
    #[serde(default)]
    pub scale: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ShapeKind {
    Box,
    Sphere,
    Circle,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ShapeConfig {
    pub kind: ShapeKind,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub dimensions: Option<DimensionsConfig>,
    #[serde(default)]
    pub radius: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ShapeOverrides {
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub dimensions: Option<DimensionsConfig>,
    #[serde(default)]
    pub radius: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PhysicsOverrides {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub body_type: Option<String>,
    #[serde(default)]
    pub mass: Option<f32>,
    #[serde(default)]
    pub restitution: Option<f32>,
    #[serde(default)]
    pub friction: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LightComponent {
    #[serde(default)]
    pub kind: Option<LightKind>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub intensity: Option<f32>,
    #[serde(default)]
    pub range: Option<f32>,
    #[serde(default)]
    pub shadows: Option<bool>,
    #[serde(default)]
    pub radius: Option<f32>,
    #[serde(default)]
    pub offset: Option<PositionConfig>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LightOverridesConfig {
    #[serde(default)]
    pub kind: Option<LightKind>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub intensity: Option<f32>,
    #[serde(default)]
    pub range: Option<f32>,
    #[serde(default)]
    pub shadows: Option<bool>,
    #[serde(default)]
    pub radius: Option<f32>,
    #[serde(default)]
    pub offset: Option<PositionConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StackConfig {
    pub count: u32,
    #[serde(default)]
    pub spacing: Vec3Config,
    #[serde(default)]
    pub start_offset: Vec3Config,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct StackOverrides {
    #[serde(default)]
    pub count: Option<u32>,
    #[serde(default)]
    pub spacing: Option<Vec3Config>,
    #[serde(default)]
    pub start_offset: Option<Vec3Config>,
}

fn default_unit_scale() -> f32 {
    1.0
}
