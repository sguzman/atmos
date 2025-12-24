use serde::Deserialize;

use super::transforms::DimensionsConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct BoundingBoxConfig {
    pub shape: String,
    pub dimensions: DimensionsConfig,
}

impl Default for BoundingBoxConfig {
    fn default() -> Self {
        Self {
            shape: "rectangle".to_string(),
            dimensions: DimensionsConfig {
                width: 100.0,
                height: 100.0,
                depth: 100.0,
            },
        }
    }
}
