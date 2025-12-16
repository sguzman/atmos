use serde::Deserialize;

use super::light::LightOverrides;
use super::shapes::RectangleOverrides;

#[derive(Debug, Deserialize)]
pub struct PillarComboConfig {
    pub name: String,
    #[serde(default)]
    pub rectangle: RectangleOverrides,
    #[serde(default)]
    pub light: LightOverrides,
}

impl Default for PillarComboConfig {
    fn default() -> Self {
        Self {
            name: "pillar_with_light".to_string(),
            rectangle: RectangleOverrides::default(),
            light: LightOverrides::default(),
        }
    }
}
