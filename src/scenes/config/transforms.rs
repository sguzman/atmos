use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Vec3Config {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}

impl Default for Vec3Config {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CubeRotationConfig {
    #[serde(default)]
    pub roll: f32,
    #[serde(default)]
    pub pitch: f32,
    #[serde(default)]
    pub yaw: f32,
}

impl Default for CubeRotationConfig {
    fn default() -> Self {
        Self {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PositionConfig {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DimensionsConfig {
    #[serde(default = "default_unit")]
    pub width: f32,
    #[serde(default = "default_unit")]
    pub height: f32,
    #[serde(default = "default_unit")]
    pub depth: f32,
}

impl Default for DimensionsConfig {
    fn default() -> Self {
        let unit = default_unit();
        Self {
            width: unit,
            height: unit,
            depth: unit,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SizeConfig {
    #[serde(default = "default_unit")]
    pub uniform_scale: f32,
}

impl Default for SizeConfig {
    fn default() -> Self {
        Self {
            uniform_scale: default_unit(),
        }
    }
}

fn default_unit() -> f32 {
    1.0
}
