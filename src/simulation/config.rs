use std::{fs, path::Path};

use bevy::prelude::{Resource, Vec3};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Resource)]
pub struct AgentDemoConfig {
    pub world: WorldConfig,
    pub camera: CameraConfig,
    pub agent: AgentConfig,
    #[serde(default)]
    pub foods: Vec<FoodConfig>,
}

impl AgentDemoConfig {
    pub fn load(path: &Path) -> Result<Self, AgentDemoConfigError> {
        let source = fs::read_to_string(path).map_err(|source| AgentDemoConfigError::Read {
            path: path.display().to_string(),
            source,
        })?;
        Self::from_toml(&source)
    }

    pub fn from_toml(source: &str) -> Result<Self, AgentDemoConfigError> {
        let config: Self = toml::from_str(source)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), AgentDemoConfigError> {
        ensure_positive("world.floor_size", self.world.floor_size)?;
        ensure_positive("agent.speed", self.agent.speed)?;
        ensure_positive("agent.perception_radius", self.agent.perception_radius)?;
        ensure_positive("agent.consume_distance", self.agent.consume_distance)?;
        ensure_positive("agent.hunger.max", self.agent.hunger.max)?;

        if self.agent.consume_distance > self.agent.perception_radius {
            return Err(AgentDemoConfigError::Invalid(
                "agent.consume_distance must not exceed agent.perception_radius".into(),
            ));
        }
        if !(0.0..=self.agent.hunger.max).contains(&self.agent.hunger.initial) {
            return Err(AgentDemoConfigError::Invalid(
                "agent.hunger.initial must be between 0 and agent.hunger.max".into(),
            ));
        }
        if !(0.0..=self.agent.hunger.max).contains(&self.agent.hunger.seek_threshold) {
            return Err(AgentDemoConfigError::Invalid(
                "agent.hunger.seek_threshold must be between 0 and agent.hunger.max".into(),
            ));
        }
        if self.agent.hunger.rate_per_second < 0.0 {
            return Err(AgentDemoConfigError::Invalid(
                "agent.hunger.rate_per_second must be non-negative".into(),
            ));
        }
        if self.foods.is_empty() {
            return Err(AgentDemoConfigError::Invalid(
                "the demo needs at least one food entity".into(),
            ));
        }
        for food in &self.foods {
            ensure_positive("foods[].nutrition", food.nutrition)?;
        }
        Ok(())
    }
}

fn ensure_positive(name: &str, value: f32) -> Result<(), AgentDemoConfigError> {
    if value <= 0.0 {
        return Err(AgentDemoConfigError::Invalid(format!(
            "{name} must be greater than zero"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorldConfig {
    pub floor_size: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CameraConfig {
    pub position: Vec3Config,
    pub look_at: Vec3Config,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub position: Vec3Config,
    pub speed: f32,
    pub perception_radius: f32,
    pub consume_distance: f32,
    pub hunger: HungerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HungerConfig {
    pub initial: f32,
    pub max: f32,
    pub rate_per_second: f32,
    pub seek_threshold: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FoodConfig {
    pub position: Vec3Config,
    pub nutrition: f32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Vec3Config {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3Config {
    pub fn as_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Error)]
pub enum AgentDemoConfigError {
    #[error("could not read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid TOML: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("invalid agent demo configuration: {0}")]
    Invalid(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_in_demo_config_is_typed_and_valid() {
        let source = include_str!("../../assets/simulation/agent_demo.toml");
        let config = AgentDemoConfig::from_toml(source).expect("demo config should be valid");

        assert!(!config.foods.is_empty());
        assert!(config.agent.perception_radius > config.agent.consume_distance);
    }

    #[test]
    fn rejects_impossible_hunger_values() {
        let source = r#"
            [world]
            floor_size = 10.0

            [camera.position]
            x = 0.0
            y = 5.0
            z = 8.0
            [camera.look_at]
            x = 0.0
            y = 0.0
            z = 0.0

            [agent]
            speed = 2.0
            perception_radius = 10.0
            consume_distance = 0.5
            [agent.position]
            x = 0.0
            y = 0.8
            z = 0.0
            [agent.hunger]
            initial = 200.0
            max = 100.0
            rate_per_second = 1.0
            seek_threshold = 50.0

            [[foods]]
            nutrition = 20.0
            [foods.position]
            x = 2.0
            y = 0.25
            z = 0.0
        "#;

        assert!(AgentDemoConfig::from_toml(source).is_err());
    }
}
