use serde::Deserialize;

use super::transforms::{DimensionsConfig, PositionConfig, Vec3Config};

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct ShootActionConfig {
    pub name: String,
    pub rate: f32,
    pub start_delay: f32,
    pub velocity: f32,
    pub spawn_offset: f32,
    #[serde(default)]
    pub ccd: bool,
    pub spin: Vec3Config,
}

impl Default for ShootActionConfig {
    fn default() -> Self {
        Self {
            name: "shoot_balls".to_string(),
            rate: 8.0,
            start_delay: 0.0,
            velocity: 18.0,
            spawn_offset: 1.2,
            ccd: false,
            spin: Vec3Config {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct GrenadeActionConfig {
    pub name: String,
    pub velocity: f32,
    pub spawn_offset: f32,
    pub fuse_seconds: f32,
    pub explosion_radius: f32,
    pub explosion_force: f32,
    pub color: String,
    pub radius: f32,
    #[serde(default)]
    pub ccd: bool,
    pub spin: Vec3Config,
}

impl Default for GrenadeActionConfig {
    fn default() -> Self {
        Self {
            name: "grenade".to_string(),
            velocity: 12.0,
            spawn_offset: 1.1,
            fuse_seconds: 1.5,
            explosion_radius: 4.0,
            explosion_force: 18.0,
            color: "green".to_string(),
            radius: 0.2,
            ccd: false,
            spin: Vec3Config {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct FovActionConfig {
    pub name: String,
    pub fov_degrees: f32,
}

impl Default for FovActionConfig {
    fn default() -> Self {
        Self {
            name: "fov".to_string(),
            fov_degrees: 60.0,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct SprintActionConfig {
    pub name: String,
    pub multiplier: f32,
    pub toggle: bool,
}

impl Default for SprintActionConfig {
    fn default() -> Self {
        Self {
            name: "sprint".to_string(),
            multiplier: 1.75,
            toggle: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct ZoomActionConfig {
    pub name: String,
    pub fov_degrees: f32,
    pub sensitivity_multiplier: f32,
    pub toggle: bool,
}

impl Default for ZoomActionConfig {
    fn default() -> Self {
        Self {
            name: "zoom".to_string(),
            fov_degrees: 25.0,
            sensitivity_multiplier: 0.4,
            toggle: false,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct JumpActionConfig {
    pub name: String,
    pub velocity: f32,
    pub cooldown: f32,
    pub ground_check_distance: f32,
}

impl Default for JumpActionConfig {
    fn default() -> Self {
        Self {
            name: "jump".to_string(),
            velocity: 6.5,
            cooldown: 0.1,
            ground_check_distance: 0.25,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct NoclipActionConfig {
    pub name: String,
    pub enabled: bool,
    pub toggle: bool,
    pub speed: f32,
    pub fast_speed: f32,
    pub speed_toggle: bool,
    pub speed_toggle_key: String,
    pub up_key: String,
    pub down_key: String,
    pub acceleration: f32,
    pub damping: f32,
}

impl Default for NoclipActionConfig {
    fn default() -> Self {
        Self {
            name: "noclip".to_string(),
            enabled: false,
            toggle: true,
            speed: 8.0,
            fast_speed: 14.0,
            speed_toggle: true,
            speed_toggle_key: "Shift".to_string(),
            up_key: "Space".to_string(),
            down_key: "lctrl".to_string(),
            acceleration: 20.0,
            damping: 8.0,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct SceneTransitionActionConfig {
    pub name: String,
    pub target_scene: String,
}

impl Default for SceneTransitionActionConfig {
    fn default() -> Self {
        Self {
            name: "scene_transition".to_string(),
            target_scene: "main".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct QuitActionConfig {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct GrabOutlineConfig {
    pub color: String,
    pub thickness: f32,
    pub opacity: f32,
}

impl Default for GrabOutlineConfig {
    fn default() -> Self {
        Self {
            color: "cyan".to_string(),
            thickness: 0.04,
            opacity: 1.0,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct GrabActionConfig {
    pub name: String,
    pub range: f32,
    pub hold_distance: f32,
    pub hold_offset: Vec3Config,
    pub throw_speed: f32,
    pub collision: bool,
    pub outline: GrabOutlineConfig,
}

impl Default for GrabActionConfig {
    fn default() -> Self {
        Self {
            name: "grab".to_string(),
            range: 6.0,
            hold_distance: 2.0,
            hold_offset: Vec3Config {
                x: 0.0,
                y: -0.1,
                z: 0.0,
            },
            throw_speed: 12.0,
            collision: false,
            outline: GrabOutlineConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct CutActionConfig {
    pub name: String,
    pub angle_step_degrees: f32,
    pub rotation_sensitivity: f32,
    pub preview_size: f32,
    pub preview_thickness: f32,
    pub preview_color: String,
    #[serde(default = "default_cut_activation_mode")]
    pub mode: CutActivationMode,
    #[serde(default = "default_cut_wheel_sensitivity")]
    pub wheel_rotation_sensitivity: f32,
    #[serde(default = "default_cut_confirm_button")]
    pub confirm_button: String,
    #[serde(default = "default_cut_preview_opacity")]
    pub preview_opacity: f32,
    #[serde(default = "default_cut_preview_emissive")]
    pub preview_emissive: f32,
}

impl Default for CutActionConfig {
    fn default() -> Self {
        Self {
            name: "cut".to_string(),
            angle_step_degrees: 15.0,
            rotation_sensitivity: 0.01,
            preview_size: 1.5,
            preview_thickness: 0.01,
            preview_color: "hotpink".to_string(),
            mode: default_cut_activation_mode(),
            wheel_rotation_sensitivity: default_cut_wheel_sensitivity(),
            confirm_button: default_cut_confirm_button(),
            preview_opacity: default_cut_preview_opacity(),
            preview_emissive: default_cut_preview_emissive(),
        }
    }
}

fn default_cut_activation_mode() -> CutActivationMode {
    CutActivationMode::Hold
}

fn default_cut_wheel_sensitivity() -> f32 {
    1.0
}

fn default_cut_confirm_button() -> String {
    "right".to_string()
}

fn default_cut_preview_opacity() -> f32 {
    0.7
}

fn default_cut_preview_emissive() -> f32 {
    2.0
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct CutAxisActionConfig {
    pub name: String,
    pub angle_step_degrees_override: Option<f32>,
    #[serde(default = "default_cut_axis_reset_angle_on_switch")]
    pub reset_angle_on_switch: bool,
}

impl Default for CutAxisActionConfig {
    fn default() -> Self {
        Self {
            name: "cut_axis".to_string(),
            angle_step_degrees_override: None,
            reset_angle_on_switch: default_cut_axis_reset_angle_on_switch(),
        }
    }
}

fn default_cut_axis_reset_angle_on_switch() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CutActivationMode {
    Hold,
    Toggle,
}

impl Default for CutActivationMode {
    fn default() -> Self {
        CutActivationMode::Hold
    }
}

impl Default for QuitActionConfig {
    fn default() -> Self {
        Self {
            name: "quit".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct ReloadActionConfig {
    pub name: String,
}

impl Default for ReloadActionConfig {
    fn default() -> Self {
        Self {
            name: "reload".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct PauseActionConfig {
    pub name: String,
    pub overlay: String,
    pub pause_scene: bool,
}

impl Default for PauseActionConfig {
    fn default() -> Self {
        Self {
            name: "pause".to_string(),
            overlay: "pause".to_string(),
            pause_scene: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DialoguePromptActionConfig {
    pub name: String,
    pub overlay: String,
}

impl Default for DialoguePromptActionConfig {
    fn default() -> Self {
        Self {
            name: "dialogue_prompt".to_string(),
            overlay: "dialogue_prompt".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DialogueActionConfig {
    pub name: String,
    pub dialogue: String,
    pub option_keys: Vec<String>,
}

impl Default for DialogueActionConfig {
    fn default() -> Self {
        Self {
            name: "dialogue".to_string(),
            dialogue: "intro".to_string(),
            option_keys: vec![
                "F1".to_string(),
                "F2".to_string(),
                "F3".to_string(),
                "F4".to_string(),
                "F5".to_string(),
                "F6".to_string(),
                "F7".to_string(),
                "F8".to_string(),
                "F9".to_string(),
            ],
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ActionsConfig {
    #[serde(default)]
    #[allow(dead_code)]
    pub version: Option<u32>,
    #[serde(default)]
    pub actions: Vec<ActionConfig>,
    #[serde(default)]
    pub triggers: Vec<ActionTriggerConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionConfig {
    Shoot {
        id: String,
        params: ShootActionConfig,
    },
    Grenade {
        id: String,
        params: GrenadeActionConfig,
    },
    Sprint {
        id: String,
        params: SprintActionConfig,
    },
    Zoom {
        id: String,
        params: ZoomActionConfig,
    },
    Jump {
        id: String,
        params: JumpActionConfig,
    },
    Noclip {
        id: String,
        params: NoclipActionConfig,
    },
    Grab {
        id: String,
        params: GrabActionConfig,
    },
    Cut {
        id: String,
        params: CutActionConfig,
    },
    CutAxis {
        id: String,
        params: CutAxisActionConfig,
    },
    Reload {
        id: String,
        #[allow(dead_code)]
        params: ReloadActionConfig,
    },
    Pause {
        id: String,
        params: PauseActionConfig,
    },
    DialoguePrompt {
        id: String,
        params: DialoguePromptActionConfig,
    },
    Dialogue {
        id: String,
        params: DialogueActionConfig,
    },
    Fov {
        id: String,
        params: FovActionConfig,
    },
    SceneTransition {
        id: String,
        params: SceneTransitionActionConfig,
    },
    Quit {
        id: String,
        #[allow(dead_code)]
        params: QuitActionConfig,
    },
}

impl ActionConfig {
    pub fn id(&self) -> &str {
        match self {
            ActionConfig::Shoot { id, .. }
            | ActionConfig::Grenade { id, .. }
            | ActionConfig::Sprint { id, .. }
            | ActionConfig::Zoom { id, .. }
            | ActionConfig::Jump { id, .. }
            | ActionConfig::Noclip { id, .. }
            | ActionConfig::Grab { id, .. }
            | ActionConfig::Cut { id, .. }
            | ActionConfig::CutAxis { id, .. }
            | ActionConfig::Reload { id, .. }
            | ActionConfig::Pause { id, .. }
            | ActionConfig::DialoguePrompt { id, .. }
            | ActionConfig::Dialogue { id, .. }
            | ActionConfig::Fov { id, .. }
            | ActionConfig::SceneTransition { id, .. }
            | ActionConfig::Quit { id, .. } => id,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TriggerMode {
    Press,
    Hold,
}

impl Default for TriggerMode {
    fn default() -> Self {
        TriggerMode::Press
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum VolumeTriggerMode {
    Enter,
    Exit,
    Inside,
}

impl Default for VolumeTriggerMode {
    fn default() -> Self {
        VolumeTriggerMode::Enter
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum VolumeShapeKind {
    Box,
    Sphere,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VolumeShapeConfig {
    pub kind: VolumeShapeKind,
    #[serde(default)]
    pub radius: Option<f32>,
    #[serde(default)]
    pub size: Option<DimensionsConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionTriggerConfig {
    Key {
        #[allow(dead_code)]
        id: String,
        action: String,
        key: String,
        #[serde(default)]
        mode: TriggerMode,
    },
    Mouse {
        #[allow(dead_code)]
        id: String,
        action: String,
        mouse: String,
        #[serde(default)]
        mode: TriggerMode,
    },
    Volume {
        #[allow(dead_code)]
        id: String,
        action: String,
        #[serde(default)]
        mode: VolumeTriggerMode,
        shape: VolumeShapeConfig,
        transform: PositionConfig,
        #[serde(default)]
        once: bool,
    },
}
