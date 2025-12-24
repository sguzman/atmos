use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct BoundingBoxConfig {
    pub shape: String,
    pub x: BoundsAxisConfig,
    pub y: BoundsAxisConfig,
    pub z: BoundsAxisConfig,
}

impl Default for BoundingBoxConfig {
    fn default() -> Self {
        Self {
            shape: "rectangle".to_string(),
            x: BoundsAxisConfig::symmetric(100.0),
            y: BoundsAxisConfig::symmetric(100.0),
            z: BoundsAxisConfig::symmetric(100.0),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct BoundsAxisConfig {
    pub min: f32,
    pub max: f32,
}

impl BoundsAxisConfig {
    fn symmetric(extent: f32) -> Self {
        Self {
            min: -extent,
            max: extent,
        }
    }
}

impl Default for BoundsAxisConfig {
    fn default() -> Self {
        Self::symmetric(100.0)
    }
}
