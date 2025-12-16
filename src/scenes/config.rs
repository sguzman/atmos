use bevy::prelude::*;
use serde::Deserialize;

pub const SCENE_ROOT: &str = "assets/scenes";

#[derive(Resource, Debug, Clone)]
pub struct ActiveScene {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InputConfig {
    #[serde(default)]
    pub camera: CameraInputConfig,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            camera: CameraInputConfig::default(),
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

#[derive(Debug, Deserialize)]
pub struct CubeConfig {
    pub name: String,
    #[serde(default = "default_color_name")]
    pub color: String,
    #[serde(default)]
    pub position: PositionConfig,
    #[serde(default)]
    pub rotation: CubeRotationConfig,
    #[serde(default)]
    pub dimensions: DimensionsConfig,
    #[serde(default)]
    pub size: SizeConfig,
    #[serde(default)]
    pub physics: PhysicsConfig,
}

impl Default for CubeConfig {
    fn default() -> Self {
        Self {
            name: "cube".to_string(),
            color: default_color_name(),
            position: PositionConfig::default(),
            rotation: CubeRotationConfig::default(),
            dimensions: DimensionsConfig::default(),
            size: SizeConfig::default(),
            physics: PhysicsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CircleConfig {
    pub name: String,
    #[serde(default = "default_circle_color_name")]
    pub color: String,
}

impl Default for CircleConfig {
    fn default() -> Self {
        Self {
            name: "base_circle".to_string(),
            color: default_circle_color_name(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub name: String,
    #[serde(default)]
    pub transform: TransformConfig,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            name: "main_camera".to_string(),
            transform: TransformConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransformConfig {
    #[serde(default = "default_camera_position")]
    pub position: Vec3Config,
    #[serde(default = "default_camera_look_at")]
    pub look_at: Vec3Config,
    #[serde(default = "default_camera_up")]
    pub up: Vec3Config,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            position: default_camera_position(),
            look_at: default_camera_look_at(),
            up: default_camera_up(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Vec3Config {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}

impl Default for Vec3Config {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CubeRotationConfig {
    #[serde(default)]
    pub roll: f32,
    #[serde(default)]
    pub pitch: f32,
    #[serde(default)]
    pub yaw: f32,
}

impl Default for CubeRotationConfig {
    fn default() -> Self {
        Self {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct PositionConfig {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}

#[derive(Debug, Deserialize)]
pub struct DimensionsConfig {
    #[serde(default = "default_unit")]
    pub width: f32,
    #[serde(default = "default_unit")]
    pub height: f32,
    #[serde(default = "default_unit")]
    pub depth: f32,
}

impl Default for DimensionsConfig {
    fn default() -> Self {
        let unit = default_unit();
        Self {
            width: unit,
            height: unit,
            depth: unit,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SizeConfig {
    #[serde(default = "default_unit")]
    pub uniform_scale: f32,
}

impl Default for SizeConfig {
    fn default() -> Self {
        Self {
            uniform_scale: default_unit(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct PhysicsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_body_type")]
    pub body_type: String,
    #[serde(default = "default_mass")]
    pub mass: f32,
    #[serde(default)]
    pub restitution: f32,
    #[serde(default = "default_friction")]
    pub friction: f32,
}

pub fn cube_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/cube.toml")
}

pub fn circle_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/circle.toml")
}

pub fn camera_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/camera.toml")
}

pub fn input_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/input.toml")
}

pub fn default_color_name() -> String {
    "red".to_string()
}

pub fn default_color_rgb() -> [u8; 3] {
    parse_color(&default_color_name()).unwrap_or([255, 0, 0])
}

pub fn parse_color(color_name: &str) -> Option<[u8; 3]> {
    match csscolorparser::parse(color_name) {
        Ok(parsed) => {
            let [r, g, b, _a] = parsed.to_rgba8();
            Some([r, g, b])
        }
        Err(err) => {
            warn!("Failed to parse color '{color_name}': {err}");
            None
        }
    }
}

fn default_unit() -> f32 {
    1.0
}

fn default_body_type() -> String {
    "dynamic".to_string()
}

fn default_mass() -> f32 {
    1.0
}

fn default_friction() -> f32 {
    0.5
}

pub fn default_circle_radius() -> f32 {
    4.0
}

pub fn default_circle_color_name() -> String {
    "white".to_string()
}

pub fn default_circle_rgb() -> [u8; 3] {
    parse_color(&default_circle_color_name()).unwrap_or([255, 255, 255])
}

fn default_camera_position() -> Vec3Config {
    Vec3Config {
        x: -2.5,
        y: 4.5,
        z: 9.0,
    }
}

fn default_camera_look_at() -> Vec3Config {
    Vec3Config {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn default_camera_up() -> Vec3Config {
    Vec3Config {
        x: 0.0,
        y: 1.0,
        z: 0.0,
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
