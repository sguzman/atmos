use bevy::{
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    prelude::{Component, Handle, Mesh, Resource, StandardMaterial},
};

use crate::scenes::config::{PhysicsConfig, ShapeConfig, ShootActionConfig, SprintActionConfig, ZoomActionConfig};

#[derive(Resource, Debug, Clone)]
pub struct SceneInputConfig {
    pub camera: ResolvedCameraInputConfig,
    pub overlays: Vec<ResolvedOverlayToggle>,
}

#[derive(Debug, Clone)]
pub struct ResolvedCameraInputConfig {
    pub movement: ResolvedMovementConfig,
    pub rotation: ResolvedRotationConfig,
}

#[derive(Debug, Clone)]
pub struct ResolvedMovementConfig {
    pub control: CameraControl,
    pub speed: f32,
    pub forward: Option<KeyCode>,
    pub backward: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
}

#[derive(Debug, Clone)]
pub struct ResolvedRotationConfig {
    pub degrees_per_second: f32,
    pub yaw_left: Option<KeyCode>,
    pub yaw_right: Option<KeyCode>,
    pub pitch_up: Option<KeyCode>,
    pub pitch_down: Option<KeyCode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraControl {
    Mouse,
    Keyboard,
}

#[derive(Debug, Clone)]
pub struct ResolvedOverlayToggle {
    pub name: String,
    pub toggle: Option<KeyCode>,
}

#[derive(Resource, Clone)]
pub struct SceneShootConfig {
    pub action: ShootActionConfig,
    pub trigger: MouseButton,
    pub name: String,
    pub shape: ShapeConfig,
    pub physics: Option<PhysicsConfig>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource, Clone)]
pub struct SceneSprintConfig {
    pub action: SprintActionConfig,
    pub trigger: KeyCode,
}

#[derive(Resource, Clone)]
pub struct SceneZoomConfig {
    pub action: ZoomActionConfig,
    pub trigger: KeyCode,
}

#[derive(Resource, Default)]
pub struct ZoomState {
    pub active: bool,
    pub base_fov: Option<f32>,
}

#[derive(Clone)]
pub struct FovBinding {
    pub trigger: KeyCode,
    pub fov_degrees: f32,
}

#[derive(Resource, Clone)]
pub struct SceneFovConfig {
    pub bindings: Vec<FovBinding>,
}

#[derive(Resource, Default)]
pub struct SprintState {
    pub active: bool,
}

#[derive(Component)]
pub struct SceneCamera;
