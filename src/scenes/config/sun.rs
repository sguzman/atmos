use serde::Deserialize;

use super::colors::default_color_name;

#[derive(Debug, Deserialize, Clone)]
pub struct SunConfig {
    #[serde(default = "default_sun_time")]
    pub time: f32, // hours 0-24
    #[serde(default = "default_color_name")]
    pub color: String,
    #[serde(default = "default_sun_brightness")]
    pub brightness: f32,
    #[serde(default = "default_sun_shadows")]
    pub shadows: bool,
    #[serde(default = "default_sun_distance")]
    pub distance: f32,
    #[serde(default = "default_sun_size")]
    pub size: f32,
}

impl Default for SunConfig {
    fn default() -> Self {
        Self {
            time: default_sun_time(),
            color: default_color_name(),
            brightness: default_sun_brightness(),
            shadows: default_sun_shadows(),
            distance: default_sun_distance(),
            size: default_sun_size(),
        }
    }
}

fn default_sun_time() -> f32 {
    12.0
}

fn default_sun_brightness() -> f32 {
    50000.0
}

fn default_sun_shadows() -> bool {
    true
}

fn default_sun_distance() -> f32 {
    50.0
}

fn default_sun_size() -> f32 {
    5.0
}
