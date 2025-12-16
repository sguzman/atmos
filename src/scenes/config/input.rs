use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputConfig {
    #[serde(default)]
    pub camera: CameraInputConfig,
    #[serde(default)]
    pub overlays: Vec<OverlayInputConfig>,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            camera: CameraInputConfig::default(),
            overlays: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CameraInputConfig {
    #[serde(default)]
    pub movement: MovementConfig,
    #[serde(default)]
    pub rotation: CameraRotationConfig,
}

impl Default for CameraInputConfig {
    fn default() -> Self {
        Self {
            movement: MovementConfig::default(),
            rotation: CameraRotationConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OverlayInputConfig {
    pub name: String,
    pub toggle: String,
}

#[derive(Debug, Deserialize)]
pub struct MovementConfig {
    #[serde(default = "default_move_speed")]
    pub speed: f32,
    #[serde(default = "default_forward_key")]
    pub forward: String,
    #[serde(default = "default_backward_key")]
    pub backward: String,
    #[serde(default = "default_left_key")]
    pub left: String,
    #[serde(default = "default_right_key")]
    pub right: String,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            speed: default_move_speed(),
            forward: default_forward_key(),
            backward: default_backward_key(),
            left: default_left_key(),
            right: default_right_key(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CameraRotationConfig {
    #[serde(default = "default_rotation_speed")]
    pub degrees_per_second: f32,
    #[serde(default = "default_yaw_left_key")]
    pub yaw_left: String,
    #[serde(default = "default_yaw_right_key")]
    pub yaw_right: String,
    #[serde(default = "default_pitch_up_key")]
    pub pitch_up: String,
    #[serde(default = "default_pitch_down_key")]
    pub pitch_down: String,
}

impl Default for CameraRotationConfig {
    fn default() -> Self {
        Self {
            degrees_per_second: default_rotation_speed(),
            yaw_left: default_yaw_left_key(),
            yaw_right: default_yaw_right_key(),
            pitch_up: default_pitch_up_key(),
            pitch_down: default_pitch_down_key(),
        }
    }
}

fn default_move_speed() -> f32 {
    6.0
}

fn default_rotation_speed() -> f32 {
    90.0
}

fn default_forward_key() -> String {
    "e".to_string()
}

fn default_backward_key() -> String {
    "d".to_string()
}

fn default_left_key() -> String {
    "s".to_string()
}

fn default_right_key() -> String {
    "f".to_string()
}

fn default_yaw_left_key() -> String {
    "ArrowLeft".to_string()
}

fn default_yaw_right_key() -> String {
    "ArrowRight".to_string()
}

fn default_pitch_up_key() -> String {
    "ArrowUp".to_string()
}

fn default_pitch_down_key() -> String {
    "ArrowDown".to_string()
}
