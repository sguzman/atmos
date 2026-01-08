use std::collections::HashMap;

use bevy::{
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    prelude::{Color, Component, Entity, Handle, Mesh, Resource, StandardMaterial, Vec3},
};

use crate::scenes::config::{
    GrabActionConfig, GrenadeActionConfig, JumpActionConfig, NoclipActionConfig, PauseActionConfig,
    PhysicsConfig, ShapeConfig, ShootActionConfig, SprintActionConfig, ZoomActionConfig,
};

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
    pub id: String,
    pub action: ShootActionConfig,
    pub name: String,
    pub shape: ShapeConfig,
    pub physics: Option<PhysicsConfig>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource, Clone)]
pub struct SceneGrenadeConfig {
    pub id: String,
    pub action: GrenadeActionConfig,
    pub name: String,
    pub shape: ShapeConfig,
    pub physics: Option<PhysicsConfig>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource, Clone)]
pub struct SceneSprintConfig {
    pub id: String,
    pub action: SprintActionConfig,
}

#[derive(Resource, Clone)]
pub struct SceneZoomConfig {
    pub id: String,
    pub action: ZoomActionConfig,
}

#[derive(Resource, Clone)]
pub struct SceneJumpConfig {
    pub id: String,
    pub action: JumpActionConfig,
}

#[derive(Resource, Clone)]
pub struct SceneNoclipConfig {
    pub id: String,
    pub action: NoclipActionConfig,
    pub speed_toggle_key: Option<KeyCode>,
    pub up_key: Option<KeyCode>,
    pub down_key: Option<KeyCode>,
}

#[derive(Resource, Clone)]
pub struct SceneGrabConfig {
    pub id: String,
    pub action: GrabActionConfig,
    pub outline_color: Color,
}

#[derive(Resource, Clone)]
pub struct SceneReloadConfig {
    pub id: String,
}

#[derive(Resource, Clone)]
pub struct ScenePauseConfig {
    pub id: String,
    pub action: PauseActionConfig,
}

#[derive(Resource, Clone)]
pub struct PauseState {
    pub active: bool,
    pub pause_scene: bool,
    pub overlay: String,
    pub stored_time_scale: f32,
}

#[derive(Resource, Default)]
pub struct ZoomState {
    pub active: bool,
    pub base_fov: Option<f32>,
}

#[derive(Clone)]
pub struct FovBinding {
    pub action_id: String,
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

#[derive(Resource, Clone)]
pub struct NoclipState {
    pub active: bool,
    pub velocity: Vec3,
    pub fast: bool,
}

#[derive(Resource, Default)]
pub struct GrabState {
    pub held: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct GrabHover {
    pub entity: Option<Entity>,
}

#[derive(Resource, Clone)]
pub struct PlayerSpawn {
    pub position: Vec3,
}

#[derive(Clone, Copy, Default)]
pub struct ActionState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

#[derive(Resource, Default)]
pub struct ActionStates {
    pub states: HashMap<String, ActionState>,
}

impl ActionStates {
    pub fn get(&self, action_id: &str) -> ActionState {
        self.states.get(action_id).copied().unwrap_or_default()
    }

    pub fn update(&mut self, action_id: &str, pressed: bool, just_pressed: bool, just_released: bool) {
        let entry = self.states.entry(action_id.to_string()).or_default();
        entry.pressed |= pressed;
        entry.just_pressed |= just_pressed;
        entry.just_released |= just_released;
    }

    pub fn clear(&mut self) {
        self.states.clear();
    }
}

#[derive(Clone)]
pub enum TriggerSource {
    Key(KeyCode),
    Mouse(MouseButton),
}

#[derive(Clone, Copy)]
pub enum TriggerMode {
    Press,
    Hold,
}

#[derive(Clone, Copy)]
pub enum VolumeTriggerMode {
    Enter,
    Exit,
    Inside,
}

#[derive(Clone)]
pub struct VolumeShape {
    pub kind: VolumeShapeKind,
    pub radius: f32,
    pub size: Vec3,
}

#[derive(Clone, Copy)]
pub enum VolumeShapeKind {
    Box,
    Sphere,
}

#[derive(Clone)]
pub struct ResolvedActionTrigger {
    pub action: String,
    pub source: TriggerSource,
    pub mode: TriggerMode,
}

#[derive(Clone)]
pub struct ResolvedVolumeTrigger {
    pub action: String,
    pub mode: VolumeTriggerMode,
    pub shape: VolumeShape,
    pub position: Vec3,
    pub once: bool,
    pub fired: bool,
    pub inside: bool,
}

#[derive(Resource, Default)]
pub struct SceneActionTriggers {
    pub input: Vec<ResolvedActionTrigger>,
    pub volumes: Vec<ResolvedVolumeTrigger>,
}

#[derive(Resource, Default, Clone)]
pub struct CameraLookState {
    pub pitch: f32,
}

#[derive(Component)]
pub struct SceneCamera;

#[derive(Component)]
pub struct PlayerBody;
