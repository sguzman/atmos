use serde::Deserialize;

use super::transforms::{Vec2Config, Vec3Config};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RenderConfig {
    #[serde(default)]
    pub tonemapping: Option<String>,
    #[serde(default)]
    pub exposure_ev100: Option<f32>,
    #[serde(default)]
    pub deband_dither: Option<bool>,
    #[serde(default)]
    pub hdr: Option<bool>,
    #[serde(default)]
    pub bloom: Option<BloomConfig>,
    #[serde(default)]
    pub fog: Option<FogConfig>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct BloomConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub intensity: Option<f32>,
    #[serde(default)]
    pub low_frequency_boost: Option<f32>,
    #[serde(default)]
    pub low_frequency_boost_curvature: Option<f32>,
    #[serde(default)]
    pub high_pass_frequency: Option<f32>,
    #[serde(default)]
    pub prefilter: Option<BloomPrefilterConfig>,
    #[serde(default)]
    pub composite_mode: Option<String>,
    #[serde(default)]
    pub max_mip_dimension: Option<u32>,
    #[serde(default)]
    pub scale: Option<Vec2Config>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct BloomPrefilterConfig {
    #[serde(default)]
    pub threshold: f32,
    #[serde(default)]
    pub threshold_softness: f32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct FogConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub alpha: Option<f32>,
    #[serde(default)]
    pub directional_light_color: Option<String>,
    #[serde(default)]
    pub directional_light_alpha: Option<f32>,
    #[serde(default)]
    pub directional_light_exponent: Option<f32>,
    #[serde(default)]
    pub falloff: Option<FogFalloffConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FogFalloffConfig {
    Linear { start: f32, end: f32 },
    Exponential { density: f32 },
    ExponentialSquared { density: f32 },
    Atmospheric { extinction: Vec3Config, inscattering: Vec3Config },
}
