use serde::Deserialize;

use super::colors::default_color_name;
use super::transforms::PositionConfig;

#[derive(Debug, Deserialize)]
pub struct LightConfig {
    #[serde(default)]
    pub lights: Vec<LightEntry>,
}

impl Default for LightConfig {
    fn default() -> Self {
        Self {
            lights: vec![LightEntry::point_default()],
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LightKind {
    Point,
    Directional,
    Ambient,
}

impl Default for LightKind {
    fn default() -> Self {
        LightKind::Point
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LightEntry {
    #[serde(default)]
    pub kind: LightKind,
    #[serde(default = "default_color_name")]
    pub color: String,
    #[serde(default = "default_light_intensity")]
    pub intensity: f32,
    #[serde(default)]
    pub range: Option<f32>,
    #[serde(default = "default_light_shadows")]
    pub shadows: bool,
    #[serde(default)]
    pub position: PositionConfig,
    #[serde(default)]
    pub look_at: Option<PositionConfig>,
    #[serde(default = "default_light_brightness")]
    pub brightness: f32, // used for ambient
    #[serde(default)]
    pub radius: Option<f32>,
    #[serde(default)]
    pub offset: PositionConfig,
}

impl LightEntry {
    pub fn point_default() -> Self {
        Self {
            kind: LightKind::Point,
            color: default_color_name(),
            intensity: default_light_intensity(),
            range: None,
            shadows: true,
            position: PositionConfig {
                x: 4.0,
                y: 8.0,
                z: 4.0,
            },
            look_at: Some(PositionConfig {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            }),
            brightness: default_light_brightness(),
            radius: None,
            offset: PositionConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LightOverrides {
    #[serde(default)]
    pub template: Option<String>,
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

fn default_light_intensity() -> f32 {
    1500.0
}

fn default_light_shadows() -> bool {
    true
}

fn default_light_brightness() -> f32 {
    0.0
}
