use serde::Deserialize;

use super::transforms::Vec3Config;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct ShootActionConfig {
    pub name: String,
    pub rate: f32,
    pub velocity: f32,
    pub spawn_offset: f32,
    pub spin: Vec3Config,
}

impl Default for ShootActionConfig {
    fn default() -> Self {
        Self {
            name: "shoot_balls".to_string(),
            rate: 8.0,
            velocity: 18.0,
            spawn_offset: 1.2,
            spin: Vec3Config {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
