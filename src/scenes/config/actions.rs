use serde::Deserialize;

use super::transforms::Vec3Config;

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
            acceleration: 20.0,
            damping: 8.0,
        }
    }
}
