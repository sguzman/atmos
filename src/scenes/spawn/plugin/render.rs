use bevy::camera::Exposure;
use bevy::camera::Hdr;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::log::warn;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::prelude::*;

use crate::scenes::config::{
    BloomConfig, DlssConfig, FogConfig, FogFalloffConfig, RayTracingConfig, RenderConfig,
};

pub(crate) fn apply_render_settings(camera: &mut EntityCommands, render: &RenderConfig) {
    let bloom_enabled = render.bloom.as_ref().is_some_and(|bloom| bloom.enabled);
    let wants_hdr = render.wants_hdr();
    if wants_hdr {
        camera.insert(Hdr);
        if render.tonemapping.is_none() {
            camera.insert(Tonemapping::default());
        }
        if render.exposure_ev100.is_none() {
            let exposure = if bloom_enabled {
                Exposure::INDOOR
            } else {
                Exposure::default()
            };
            camera.insert(exposure);
        }
    }

    if let Some(tonemapping) = render.tonemapping.as_deref().and_then(parse_tonemapping) {
        camera.insert(tonemapping);
    }

    if let Some(ev100) = render.exposure_ev100 {
        camera.insert(Exposure { ev100 });
    }

    if let Some(enabled) = render.deband_dither {
        let dither = if enabled {
            DebandDither::Enabled
        } else {
            DebandDither::Disabled
        };
        camera.insert(dither);
    }

    if let Some(bloom) = render.bloom.as_ref().filter(|bloom| bloom.enabled) {
        camera.insert(resolve_bloom(bloom));
    }

    if let Some(fog) = render.fog.as_ref().filter(|fog| fog.enabled) {
        camera.insert(resolve_fog(fog));
    }

    if let Some(dlss) = render.dlss.as_ref().filter(|dlss| dlss.enabled) {
        warn_dlss_unavailable(dlss);
    }
    if let Some(ray) = render.ray_tracing.as_ref().filter(|ray| ray.enabled) {
        warn_ray_tracing_unavailable(ray);
    }
}

fn warn_dlss_unavailable(config: &DlssConfig) {
    let mode = config.mode.as_deref().unwrap_or("default");
    let sharpness = config.sharpness.unwrap_or(0.0);
    warn!("DLSS requested (mode={mode}, sharpness={sharpness}) but no DLSS backend is configured.");
}

fn warn_ray_tracing_unavailable(config: &RayTracingConfig) {
    let mode = config.mode.as_deref().unwrap_or("default");
    warn!(
        "Ray tracing requested (mode={mode}) but Bevy's renderer does not expose ray tracing here."
    );
}

fn parse_tonemapping(value: &str) -> Option<Tonemapping> {
    let normalized = value.trim().to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        "none" => Some(Tonemapping::None),
        "reinhard" => Some(Tonemapping::Reinhard),
        "reinhard_luminance" => Some(Tonemapping::ReinhardLuminance),
        "aces_fitted" => Some(Tonemapping::AcesFitted),
        "agx" => Some(Tonemapping::AgX),
        "somewhat_boring_display_transform" => Some(Tonemapping::SomewhatBoringDisplayTransform),
        "tony_mc_mapface" => Some(Tonemapping::TonyMcMapface),
        "blender_filmic" => Some(Tonemapping::BlenderFilmic),
        _ => None,
    }
}

fn resolve_bloom(config: &BloomConfig) -> Bloom {
    let mut bloom = match config
        .preset
        .as_deref()
        .map(|preset| preset.trim().to_ascii_lowercase().replace('-', "_"))
        .as_deref()
    {
        Some("natural") => Bloom::NATURAL,
        Some("old_school") => Bloom::OLD_SCHOOL,
        Some("screen_blur") => Bloom::SCREEN_BLUR,
        _ => Bloom::default(),
    };

    if let Some(value) = config.intensity {
        bloom.intensity = value;
    }
    if let Some(value) = config.low_frequency_boost {
        bloom.low_frequency_boost = value;
    }
    if let Some(value) = config.low_frequency_boost_curvature {
        bloom.low_frequency_boost_curvature = value;
    }
    if let Some(value) = config.high_pass_frequency {
        bloom.high_pass_frequency = value;
    }
    if let Some(prefilter) = config.prefilter.as_ref() {
        bloom.prefilter = BloomPrefilter {
            threshold: prefilter.threshold,
            threshold_softness: prefilter.threshold_softness,
        };
    }
    if let Some(mode) = config
        .composite_mode
        .as_deref()
        .map(|mode| mode.trim().to_ascii_lowercase().replace('-', "_"))
    {
        bloom.composite_mode = match mode.as_str() {
            "additive" => BloomCompositeMode::Additive,
            _ => BloomCompositeMode::EnergyConserving,
        };
    }
    if let Some(value) = config.max_mip_dimension {
        bloom.max_mip_dimension = value;
    }
    if let Some(scale) = config.scale.as_ref() {
        bloom.scale = Vec2::new(scale.x, scale.y);
    }

    bloom
}

fn resolve_fog(config: &FogConfig) -> DistanceFog {
    let color = config
        .color
        .as_deref()
        .and_then(crate::scenes::config::parse_color)
        .unwrap_or([255, 255, 255]);
    let alpha = config.alpha.unwrap_or(1.0).clamp(0.0, 1.0);
    let fog_color = Color::srgba_u8(color[0], color[1], color[2], (alpha * 255.0) as u8);

    let directional_light_color = if let Some(color) = config
        .directional_light_color
        .as_deref()
        .and_then(crate::scenes::config::parse_color)
    {
        let alpha = config
            .directional_light_alpha
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        Color::srgba_u8(color[0], color[1], color[2], (alpha * 255.0) as u8)
    } else {
        Color::NONE
    };

    let falloff = match config.falloff.as_ref() {
        Some(FogFalloffConfig::Linear { start, end }) => FogFalloff::Linear {
            start: *start,
            end: *end,
        },
        Some(FogFalloffConfig::Exponential { density }) => {
            FogFalloff::Exponential { density: *density }
        }
        Some(FogFalloffConfig::ExponentialSquared { density }) => {
            FogFalloff::ExponentialSquared { density: *density }
        }
        Some(FogFalloffConfig::Atmospheric {
            extinction,
            inscattering,
        }) => FogFalloff::Atmospheric {
            extinction: Vec3::new(extinction.x, extinction.y, extinction.z),
            inscattering: Vec3::new(inscattering.x, inscattering.y, inscattering.z),
        },
        None => FogFalloff::Linear {
            start: 0.0,
            end: 100.0,
        },
    };

    DistanceFog {
        color: fog_color,
        directional_light_color,
        directional_light_exponent: config.directional_light_exponent.unwrap_or(8.0),
        falloff,
    }
}
