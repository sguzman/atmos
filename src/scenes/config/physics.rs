use serde::Deserialize;

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

fn default_body_type() -> String {
    "dynamic".to_string()
}

fn default_mass() -> f32 {
    1.0
}

fn default_friction() -> f32 {
    0.5
}
