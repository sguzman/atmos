use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy_rapier3d::prelude::{DefaultRapierContext, RapierConfiguration};

use crate::scenes::input::{DebugMenuState, SceneCamera, ZoomState};
use crate::scenes::spawn::SunLight;

use super::types::{DebugMenuAction, DebugMenuSliderKind};

pub(crate) fn apply_debug_menu_action(
    action: &DebugMenuAction,
    debug_state: &mut DebugMenuState,
    commands: &mut Commands,
    camera_entities: &Query<Entity, With<SceneCamera>>,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    match action {
        DebugMenuAction::Noop => {}
        DebugMenuAction::Open(page) => {
            debug_state.stack.push(*page);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::Back => {
            if debug_state.stack.len() > 1 {
                debug_state.stack.pop();
                debug_state.needs_refresh = true;
            }
        }
        DebugMenuAction::ToggleBloom => {
            debug_state.settings.bloom_enabled = !debug_state.settings.bloom_enabled;
            apply_bloom_settings(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleFog => {
            debug_state.settings.fog_enabled = !debug_state.settings.fog_enabled;
            apply_fog_settings(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleDlss => {
            debug_state.settings.dlss_enabled = !debug_state.settings.dlss_enabled;
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::CycleDlssMode => {
            debug_state.settings.dlss_mode = next_quality_mode(&debug_state.settings.dlss_mode);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::CycleFogMode => {
            debug_state.settings.fog_mode = next_fog_mode(&debug_state.settings.fog_mode);
            apply_fog_settings(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleRayTracing => {
            debug_state.settings.ray_tracing_enabled = !debug_state.settings.ray_tracing_enabled;
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::CycleRayTracingMode => {
            debug_state.settings.ray_tracing_mode =
                next_quality_mode(&debug_state.settings.ray_tracing_mode);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::TogglePhysics => {
            debug_state.settings.physics_enabled = !debug_state.settings.physics_enabled;
            apply_physics_toggle(debug_state, rapier_config);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleSunShadows => {
            if debug_state.settings.sun_present {
                debug_state.settings.sun_shadows = !debug_state.settings.sun_shadows;
                apply_sun(debug_state, sun);
                debug_state.needs_refresh = true;
            }
        }
    }
}

pub(crate) fn apply_slider_value(
    kind: DebugMenuSliderKind,
    value: f32,
    debug_state: &mut DebugMenuState,
    commands: &mut Commands,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
    camera_entities: &Query<Entity, With<SceneCamera>>,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    match kind {
        DebugMenuSliderKind::Fov => {
            debug_state.settings.fov_degrees = value;
            apply_fov(debug_state, projections, zoom_state);
        }
        DebugMenuSliderKind::GravityY => {
            debug_state.settings.gravity.y = value;
            apply_gravity(debug_state, rapier_config);
        }
        DebugMenuSliderKind::SunBrightness => {
            debug_state.settings.sun_brightness = value;
            apply_sun(debug_state, sun);
        }
        DebugMenuSliderKind::DlssSharpness => {
            debug_state.settings.dlss_sharpness = value;
        }
        DebugMenuSliderKind::BloomIntensity => {
            debug_state.settings.bloom_intensity = value;
            apply_bloom_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::BloomThreshold => {
            debug_state.settings.bloom_threshold = value;
            apply_bloom_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::BloomThresholdSoftness => {
            debug_state.settings.bloom_threshold_softness = value;
            apply_bloom_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogAlpha => {
            debug_state.settings.fog_alpha = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogDensity => {
            debug_state.settings.fog_density = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogLinearStart => {
            debug_state.settings.fog_linear_start = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogLinearEnd => {
            debug_state.settings.fog_linear_end = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
    }
}

fn apply_fov(
    debug_state: &DebugMenuState,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
) {
    let fov_radians = debug_state.settings.fov_degrees.to_radians();
    if let Some(zoom_state) = zoom_state.as_mut() {
        zoom_state.base_fov = Some(fov_radians);
        if zoom_state.active {
            return;
        }
    }
    for mut projection in projections.iter_mut() {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = fov_radians;
        }
    }
}

fn apply_bloom_settings(
    debug_state: &DebugMenuState,
    commands: &mut Commands,
    cameras: &Query<Entity, With<SceneCamera>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    if debug_state.settings.bloom_enabled {
        let mut bloom = debug_state
            .settings
            .bloom
            .clone()
            .unwrap_or_else(Bloom::default);
        bloom.intensity = debug_state.settings.bloom_intensity;
        bloom.prefilter.threshold = debug_state.settings.bloom_threshold;
        bloom.prefilter.threshold_softness = debug_state.settings.bloom_threshold_softness;
        commands.entity(camera).insert(bloom);
    } else {
        commands.entity(camera).remove::<Bloom>();
    }
}

fn apply_fog_settings(
    debug_state: &DebugMenuState,
    commands: &mut Commands,
    cameras: &Query<Entity, With<SceneCamera>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    if debug_state.settings.fog_enabled {
        let mut fog = debug_state
            .settings
            .fog
            .clone()
            .unwrap_or_else(DistanceFog::default);
        let alpha = debug_state.settings.fog_alpha.clamp(0.0, 1.0);
        fog.color.set_alpha(alpha);
        fog.falloff = match debug_state.settings.fog_mode.as_str() {
            "exponential" => FogFalloff::Exponential {
                density: debug_state.settings.fog_density,
            },
            "exponential_squared" => FogFalloff::ExponentialSquared {
                density: debug_state.settings.fog_density,
            },
            _ => FogFalloff::Linear {
                start: debug_state.settings.fog_linear_start,
                end: debug_state.settings.fog_linear_end,
            },
        };
        commands.entity(camera).insert(fog);
    } else {
        commands.entity(camera).remove::<DistanceFog>();
    }
}

fn apply_gravity(
    debug_state: &DebugMenuState,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = debug_state.settings.gravity;
    }
}

fn apply_physics_toggle(
    debug_state: &DebugMenuState,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if let Ok(mut config) = rapier_config.single_mut() {
        config.physics_pipeline_active = debug_state.settings.physics_enabled;
    }
}

fn apply_sun(debug_state: &DebugMenuState, sun: &mut Query<&mut DirectionalLight, With<SunLight>>) {
    if let Ok(mut light) = sun.single_mut() {
        light.illuminance = debug_state.settings.sun_brightness;
        light.shadows_enabled = debug_state.settings.sun_shadows;
    }
}

fn next_quality_mode(current: &str) -> String {
    match current.trim().to_ascii_lowercase().as_str() {
        "performance" => "balanced".to_string(),
        "balanced" => "quality".to_string(),
        _ => "performance".to_string(),
    }
}

fn next_fog_mode(current: &str) -> String {
    match current.trim().to_ascii_lowercase().as_str() {
        "exponential" => "exponential_squared".to_string(),
        "exponential_squared" => "linear".to_string(),
        _ => "exponential".to_string(),
    }
}
