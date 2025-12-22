use serde::Deserialize;

use super::colors::{default_circle_color_name, default_color_name};
use super::physics::PhysicsConfig;
use super::transforms::{CubeRotationConfig, DimensionsConfig, PositionConfig, SizeConfig};

#[derive(Debug, Deserialize)]
pub struct CubeConfig {
    pub name: String,
    #[serde(default = "default_color_name")]
    pub color: String,
    #[serde(default)]
    pub position: PositionConfig,
    #[serde(default)]
    pub rotation: CubeRotationConfig,
    #[serde(default)]
    pub dimensions: DimensionsConfig,
    #[serde(default)]
    pub size: SizeConfig,
    #[serde(default)]
    pub physics: PhysicsConfig,
}

impl Default for CubeConfig {
    fn default() -> Self {
        Self {
            name: "cube".to_string(),
            color: default_color_name(),
            position: PositionConfig::default(),
            rotation: CubeRotationConfig::default(),
            dimensions: DimensionsConfig::default(),
            size: SizeConfig::default(),
            physics: PhysicsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CircleConfig {
    pub name: String,
    #[serde(default = "default_circle_color_name")]
    pub color: String,
    #[serde(default)]
    pub physics: PhysicsConfig,
}

impl Default for CircleConfig {
    fn default() -> Self {
        Self {
            name: "base_circle".to_string(),
            color: default_circle_color_name(),
            physics: PhysicsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RectangleConfig {
    pub name: String,
    #[serde(default = "default_color_name")]
    pub color: String,
    #[serde(default)]
    pub dimensions: DimensionsConfig,
    #[serde(default)]
    pub physics: PhysicsConfig,
}

impl Default for RectangleConfig {
    fn default() -> Self {
        Self {
            name: "rectangle".to_string(),
            color: default_color_name(),
            dimensions: DimensionsConfig {
                width: 1.0,
                height: 3.0,
                depth: 0.5,
            },
            physics: PhysicsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RectangleOverrides {
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub dimensions: Option<DimensionsConfig>,
}
