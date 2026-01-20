use std::collections::HashMap;

use bevy::{
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    pbr::DistanceFog,
    post_process::bloom::Bloom,
    prelude::{
        Color, Component, Entity,
        Handle, Mesh, Resource,
        StandardMaterial, Vec3,
    },
};

use crate::scenes::config::{
    CutActionConfig, CutAxisActionConfig,
    DialogueConfig,
    GrabActionConfig,
    GrenadeActionConfig,
    JumpActionConfig,
    NoclipActionConfig,
    PauseActionConfig, PhysicsConfig,
    ShapeConfig, ShootActionConfig,
    SprintActionConfig,
    ZoomActionConfig,
};

#[derive(Resource, Debug, Clone)]
pub struct SceneInputConfig {
    pub camera:
        ResolvedCameraInputConfig,
    pub overlays:
        Vec<ResolvedOverlayToggle>,
}

#[derive(Debug, Clone)]
pub struct ResolvedCameraInputConfig {
    pub movement:
        ResolvedMovementConfig,
    pub rotation:
        ResolvedRotationConfig,
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq,
)]
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
    pub material:
        Handle<StandardMaterial>,
}

#[derive(Resource, Clone)]
pub struct SceneGrenadeConfig {
    pub id: String,
    pub action: GrenadeActionConfig,
    pub name: String,
    pub shape: ShapeConfig,
    pub physics: Option<PhysicsConfig>,
    pub mesh: Handle<Mesh>,
    pub material:
        Handle<StandardMaterial>,
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
    pub speed_toggle_key:
        Option<KeyCode>,
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
pub struct SceneCutConfig {
    pub id: String,
    pub action: CutActionConfig,
    pub confirm_button: MouseButton,
}

#[derive(Resource, Clone)]
pub struct SceneCutAxisConfig {
    pub id: String,
    pub action: CutAxisActionConfig,
}

#[derive(Resource, Clone)]
pub struct SceneReloadConfig {
    pub id: String,
}

#[derive(Resource, Clone)]
pub struct ScenePauseConfig {
    pub id: String,
    #[allow(dead_code)]
    pub action: PauseActionConfig,
}

#[derive(Resource, Clone)]
pub struct PauseState {
    pub active: bool,
    pub pause_scene: bool,
    pub overlay: String,
    pub stored_time_scale: f32,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum DebugMenuPage {
    Root,
    Camera,
    Render,
    RenderDlss,
    RenderBloom,
    RenderFog,
    RenderRayTracing,
    Physics,
    Sun,
}

impl Default for DebugMenuPage {
    fn default() -> Self {
        DebugMenuPage::Root
    }
}

#[derive(Clone)]
pub struct DebugMenuSettings {
    pub initialized: bool,
    pub fov_degrees: f32,
    pub bloom_enabled: bool,
    pub bloom: Option<Bloom>,
    pub bloom_intensity: f32,
    pub bloom_threshold: f32,
    pub bloom_threshold_softness: f32,
    pub fog_enabled: bool,
    pub fog: Option<DistanceFog>,
    pub fog_mode: String,
    pub fog_alpha: f32,
    pub fog_density: f32,
    pub fog_linear_start: f32,
    pub fog_linear_end: f32,
    pub dlss_enabled: bool,
    pub dlss_mode: String,
    pub dlss_sharpness: f32,
    pub ray_tracing_enabled: bool,
    pub ray_tracing_mode: String,
    pub gravity: Vec3,
    pub physics_enabled: bool,
    pub sun_brightness: f32,
    pub sun_shadows: bool,
    pub sun_present: bool,
}

impl Default for DebugMenuSettings {
    fn default() -> Self {
        Self {
            initialized: false,
            fov_degrees: 60.0,
            bloom_enabled: false,
            bloom: None,
            bloom_intensity: 0.15,
            bloom_threshold: 0.9,
            bloom_threshold_softness:
                0.0,
            fog_enabled: false,
            fog: None,
            fog_mode: "linear"
                .to_string(),
            fog_alpha: 1.0,
            fog_density: 0.05,
            fog_linear_start: 0.0,
            fog_linear_end: 100.0,
            dlss_enabled: false,
            dlss_mode: "quality"
                .to_string(),
            dlss_sharpness: 0.0,
            ray_tracing_enabled: false,
            ray_tracing_mode: "quality"
                .to_string(),
            gravity: Vec3::new(
                0.0, -9.81, 0.0,
            ),
            physics_enabled: true,
            sun_brightness: 0.0,
            sun_shadows: false,
            sun_present: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct DebugMenuState {
    pub active: bool,
    pub stack: Vec<DebugMenuPage>,
    pub settings: DebugMenuSettings,
    pub stored_time_scale: f32,
    pub needs_refresh: bool,
    pub active_slider: Option<Entity>,
}

#[derive(Resource, Clone)]
pub struct SceneDialogueConfig {
    pub prompt_action_id: String,
    pub interact_action_id: String,
    pub prompt_overlay: String,
    pub dialogue: String,
    pub option_keys: Vec<KeyCode>,
    pub option_labels: Vec<String>,
}

#[derive(Resource, Default)]
pub struct DialogueState {
    pub active: bool,
    pub pending: bool,
    pub current: String,
    pub visited:
        std::collections::HashSet<
            String,
        >,
    pub dialogue:
        Option<DialogueConfig>,
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

#[derive(Resource, Default)]
pub struct CutHover {
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
    pub states:
        HashMap<String, ActionState>,
}

impl ActionStates {
    pub fn get(
        &self,
        action_id: &str,
    ) -> ActionState {
        self.states
            .get(action_id)
            .copied()
            .unwrap_or_default()
    }

    pub fn update(
        &mut self,
        action_id: &str,
        pressed: bool,
        just_pressed: bool,
        just_released: bool,
    ) {
        let entry = self
            .states
            .entry(
                action_id.to_string(),
            )
            .or_default();
        entry.pressed |= pressed;
        entry.just_pressed |=
            just_pressed;
        entry.just_released |=
            just_released;
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
    pub input:
        Vec<ResolvedActionTrigger>,
    pub volumes:
        Vec<ResolvedVolumeTrigger>,
}

#[derive(Resource, Default, Clone)]
pub struct CameraLookState {
    pub pitch: f32,
}

#[derive(Component)]
pub struct SceneCamera;

#[derive(Component)]
pub struct PlayerBody;
