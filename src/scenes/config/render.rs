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
    #[serde(default)]
    pub dlss: Option<DlssConfig>,
    #[serde(default)]
    pub ray_tracing: Option<RayTracingConfig>,
    #[serde(default)]
    pub clouds: Option<VolumetricCloudsConfig>,
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

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DlssConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub sharpness: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RayTracingConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct VolumetricCloudsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub raymarch_steps: Option<u32>,
    #[serde(default)]
    pub shadow_raymarch_steps: Option<u32>,
    #[serde(default)]
    pub planet_radius: Option<f32>,
    #[serde(default)]
    pub bottom_height: Option<f32>,
    #[serde(default)]
    pub top_height: Option<f32>,
    #[serde(default)]
    pub coverage: Option<f32>,
    #[serde(default)]
    pub density: Option<f32>,
    #[serde(default)]
    pub base_scale: Option<f32>,
    #[serde(default)]
    pub detail_scale: Option<f32>,
    #[serde(default)]
    pub detail_strength: Option<f32>,
    #[serde(default)]
    pub base_edge_softness: Option<f32>,
    #[serde(default)]
    pub bottom_softness: Option<f32>,
    #[serde(default)]
    pub shadow_step_size: Option<f32>,
    #[serde(default)]
    pub shadow_step_multiply: Option<f32>,
    #[serde(default)]
    pub forward_scattering_g: Option<f32>,
    #[serde(default)]
    pub backward_scattering_g: Option<f32>,
    #[serde(default)]
    pub scattering_lerp: Option<f32>,
    #[serde(default)]
    pub ambient_color_top: Option<String>,
    #[serde(default)]
    pub ambient_color_bottom: Option<String>,
    #[serde(default)]
    pub min_transmittance: Option<f32>,
    #[serde(default)]
    pub wind_velocity: Option<Vec3Config>,
    #[serde(default)]
    pub render_resolution: Option<Vec2Config>,
    #[serde(default)]
    pub god_rays: Option<GodRaysConfig>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct GodRaysConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub intensity: Option<f32>,
    #[serde(default)]
    pub decay: Option<f32>,
    #[serde(default)]
    pub exposure: Option<f32>,
    #[serde(default)]
    pub samples: Option<u32>,
}
