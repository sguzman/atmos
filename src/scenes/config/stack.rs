use serde::Deserialize;

use super::transforms::Vec3Config;
use super::RectangleOverrides;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct RectangleStackConfig {
    pub name: String,
    pub count: u32,
    pub spacing: Vec3Config,
    pub start_offset: Vec3Config,
    #[serde(default)]
    pub rectangle: RectangleOverrides,
}

impl Default for RectangleStackConfig {
    fn default() -> Self {
        Self {
            name: "rectangle_stack".to_string(),
            count: 5,
            spacing: Vec3Config { x: 0.0, y: 4.1, z: 0.0 },
            start_offset: Vec3Config { x: 0.0, y: 0.0, z: 0.0 },
            rectangle: RectangleOverrides::default(),
        }
    }
}
