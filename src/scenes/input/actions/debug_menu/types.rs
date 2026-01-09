use bevy::prelude::*;

use crate::scenes::input::DebugMenuPage;

#[derive(Component)]
pub struct DebugMenuUiTag;

#[derive(Component, Clone)]
pub(crate) struct DebugMenuButton {
    pub(crate) action: DebugMenuAction,
}

#[derive(Component, Clone)]
pub(crate) struct DebugMenuSlider {
    pub(crate) kind: DebugMenuSliderKind,
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) fill: Entity,
}

#[derive(Component, Clone)]
pub(crate) struct DebugMenuSliderLabel {
    pub(crate) kind: DebugMenuSliderKind,
    pub(crate) label: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DebugMenuSliderKind {
    Fov,
    GravityY,
    SunBrightness,
    DlssSharpness,
    BloomIntensity,
    BloomThreshold,
    BloomThresholdSoftness,
    FogAlpha,
    FogDensity,
    FogLinearStart,
    FogLinearEnd,
}

#[derive(Clone)]
pub(crate) enum DebugMenuAction {
    Noop,
    Open(DebugMenuPage),
    Back,
    ToggleBloom,
    ToggleFog,
    ToggleDlss,
    CycleDlssMode,
    CycleFogMode,
    ToggleRayTracing,
    CycleRayTracingMode,
    TogglePhysics,
    ToggleSunShadows,
}

pub(crate) struct DebugMenuEntry {
    pub(crate) label: String,
    pub(crate) action: DebugMenuAction,
}

pub(crate) struct DebugMenuSliderConfig {
    pub(crate) label: String,
    pub(crate) kind: DebugMenuSliderKind,
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) value: f32,
}

impl DebugMenuSliderConfig {
    pub(crate) fn new(
        label: &str,
        kind: DebugMenuSliderKind,
        min: f32,
        max: f32,
        value: f32,
    ) -> Self {
        Self {
            label: label.to_string(),
            kind,
            min,
            max,
            value,
        }
    }
}
