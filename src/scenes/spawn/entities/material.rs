use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use bevy::render::alpha::AlphaMode;

use crate::scenes::config::{parse_color, ActiveScene, MaterialConfig, ShapeConfig};

pub(super) fn resolve_material(
    shape: &ShapeConfig,
    material: Option<&MaterialConfig>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) -> Handle<StandardMaterial> {
    let shape_color = super::shape::resolve_shape_color(shape, active_scene);
    let base_color_fallback = Color::srgb_u8(shape_color[0], shape_color[1], shape_color[2]);
    let mut resolved = if let Some(material) = material {
        resolve_material_from_config(material, base_color_fallback, asset_server)
    } else {
        let mut material = StandardMaterial::default();
        material.base_color = base_color_fallback;
        material
    };

    if resolved.alpha_mode == AlphaMode::Opaque {
        if resolved.base_color.to_srgba().alpha < 1.0 {
            resolved.alpha_mode = AlphaMode::Blend;
        }
    }

    materials.add(resolved)
}

fn resolve_material_from_config(
    config: &MaterialConfig,
    base_color_fallback: Color,
    asset_server: &AssetServer,
) -> StandardMaterial {
    let mut material = match config
        .preset
        .as_deref()
        .map(|preset| preset.trim().to_ascii_lowercase().replace('-', "_"))
        .as_deref()
    {
        Some("wood") | Some("wooden") => preset_wood(),
        Some("metal") | Some("metallic") => preset_metal(),
        Some("marble") => preset_marble(),
        Some("stone") => preset_stone(),
        Some("glass") => preset_glass(),
        _ => StandardMaterial::default(),
    };

    if let Some(color) = config.base_color.as_deref().and_then(parse_color) {
        material.base_color = Color::srgb_u8(color[0], color[1], color[2]);
    } else if config.preset.is_none() {
        material.base_color = base_color_fallback;
    }

    if let Some(opacity) = config.opacity {
        let mut color = material.base_color;
        color.set_alpha(opacity.clamp(0.0, 1.0));
        material.base_color = color;
    }

    if let Some(path) = config.base_color_texture.as_deref() {
        material.base_color_texture = Some(load_texture(asset_server, path, true));
    }

    if let Some(metallic) = config.metallic {
        material.metallic = metallic.clamp(0.0, 1.0);
    }
    if let Some(roughness) = config.roughness {
        material.perceptual_roughness = roughness.clamp(0.0, 1.0);
    }
    if let Some(reflectance) = config.reflectance {
        material.reflectance = reflectance.clamp(0.0, 1.0);
    }
    if let Some(color) = config.specular_tint.as_deref().and_then(parse_color) {
        material.specular_tint = Color::srgb_u8(color[0], color[1], color[2]);
    }

    if let Some(color) = config.emissive_color.as_deref().and_then(parse_color) {
        let intensity = config.emissive_intensity.unwrap_or(1.0);
        let base = Color::srgb_u8(color[0], color[1], color[2]);
        material.emissive = base.to_linear() * intensity;
    } else if let Some(intensity) = config.emissive_intensity {
        material.emissive = Color::WHITE.to_linear() * intensity;
    }

    if let Some(path) = config.emissive_texture.as_deref() {
        material.emissive_texture = Some(load_texture(asset_server, path, true));
    }

    if let Some(path) = config.normal_map.as_deref() {
        material.normal_map_texture = Some(load_texture(asset_server, path, false));
    }
    if let Some(flip) = config.flip_normal_map_y {
        material.flip_normal_map_y = flip;
    }
    if let Some(path) = config.metallic_roughness_texture.as_deref() {
        material.metallic_roughness_texture = Some(load_texture(asset_server, path, false));
    }
    if let Some(path) = config.occlusion_texture.as_deref() {
        material.occlusion_texture = Some(load_texture(asset_server, path, false));
    }

    if let Some(alpha_mode) = config.alpha_mode.as_deref() {
        material.alpha_mode = match alpha_mode.trim().to_ascii_lowercase().as_str() {
            "blend" => AlphaMode::Blend,
            "premultiplied" => AlphaMode::Premultiplied,
            "add" => AlphaMode::Add,
            "multiply" => AlphaMode::Multiply,
            "mask" => AlphaMode::Mask(config.alpha_cutoff.unwrap_or(0.5)),
            _ => AlphaMode::Opaque,
        };
    } else if config.opacity.unwrap_or(1.0) < 1.0 {
        material.alpha_mode = AlphaMode::Blend;
    }

    if let Some(unlit) = config.unlit {
        material.unlit = unlit;
    }
    if let Some(double_sided) = config.double_sided {
        material.double_sided = double_sided;
    }

    if let Some(clearcoat) = config.clearcoat {
        material.clearcoat = clearcoat.clamp(0.0, 1.0);
    }
    if let Some(roughness) = config.clearcoat_roughness {
        material.clearcoat_perceptual_roughness = roughness.clamp(0.0, 1.0);
    }
    if let Some(ior) = config.ior {
        material.ior = ior;
    }
    if let Some(transmission) = config.specular_transmission {
        material.specular_transmission = transmission.clamp(0.0, 1.0);
    }
    if let Some(transmission) = config.diffuse_transmission {
        material.diffuse_transmission = transmission.clamp(0.0, 1.0);
    }
    if let Some(thickness) = config.thickness {
        material.thickness = thickness.max(0.0);
    }
    if let Some(color) = config.attenuation_color.as_deref().and_then(parse_color) {
        material.attenuation_color = Color::srgb_u8(color[0], color[1], color[2]);
    }
    if let Some(distance) = config.attenuation_distance {
        material.attenuation_distance = distance.max(0.0);
    }

    material
}

fn load_texture(
    asset_server: &AssetServer,
    path: &str,
    is_srgb: bool,
) -> Handle<Image> {
    asset_server.load_with_settings(
        path.to_string(),
        move |settings: &mut ImageLoaderSettings| settings.is_srgb = is_srgb,
    )
}

fn preset_wood() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(140, 96, 64),
        metallic: 0.0,
        perceptual_roughness: 0.7,
        reflectance: 0.5,
        ..default()
    }
}

fn preset_metal() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(180, 180, 190),
        metallic: 1.0,
        perceptual_roughness: 0.2,
        reflectance: 0.9,
        ..default()
    }
}

fn preset_marble() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(235, 232, 225),
        metallic: 0.0,
        perceptual_roughness: 0.25,
        reflectance: 0.6,
        ..default()
    }
}

fn preset_stone() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(120, 120, 120),
        metallic: 0.0,
        perceptual_roughness: 0.85,
        reflectance: 0.4,
        ..default()
    }
}

fn preset_glass() -> StandardMaterial {
    let mut material = StandardMaterial {
        base_color: Color::srgba_u8(200, 220, 230, 20),
        metallic: 0.0,
        perceptual_roughness: 0.05,
        reflectance: 0.9,
        specular_transmission: 1.0,
        thickness: 0.5,
        ior: 1.5,
        ..default()
    };
    material.alpha_mode = AlphaMode::Blend;
    material
}
