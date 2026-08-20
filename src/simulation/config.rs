use std::{collections::BTreeSet, fs, path::Path};

use bevy::prelude::{Color, Resource, Vec3};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Resource)]
pub struct AgentDemoConfig {
    pub showcase: ShowcaseConfig,
    pub world: WorldConfig,
    pub camera: CameraConfig,
    pub agent: AgentConfig,
    #[serde(default)]
    pub foods: Vec<FoodConfig>,
    #[serde(default)]
    pub scripted_events: Vec<ScriptedEventConfig>,
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
        ensure_positive("agent.motion.speed", self.agent.motion.speed)?;
        ensure_positive(
            "agent.motion.consume_distance",
            self.agent.motion.consume_distance,
        )?;
        ensure_positive("agent.perception_radius", self.agent.perception_radius)?;
        ensure_positive("agent.hunger.max", self.agent.hunger.max)?;

        if self.agent.motion.consume_distance > self.agent.perception_radius {
            return Err(AgentDemoConfigError::Invalid(
                "agent.motion.consume_distance must not exceed agent.perception_radius".into(),
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
                "the showcase needs at least one food entity".into(),
            ));
        }
        if self.showcase.name.trim().is_empty() {
            return Err(AgentDemoConfigError::Invalid(
                "showcase.name must not be empty".into(),
            ));
        }
        if self.showcase.description.trim().is_empty() {
            return Err(AgentDemoConfigError::Invalid(
                "showcase.description must not be empty".into(),
            ));
        }

        let mut food_ids = BTreeSet::new();
        for food in &self.foods {
            ensure_positive("foods[].nutrition", food.nutrition)?;
            if food.id.trim().is_empty() {
                return Err(AgentDemoConfigError::Invalid(
                    "foods[].id must not be empty".into(),
                ));
            }
            if !food_ids.insert(food.id.clone()) {
                return Err(AgentDemoConfigError::Invalid(format!(
                    "duplicate food id '{}'",
                    food.id
                )));
            }
        }

        for event in &self.scripted_events {
            if event.at_seconds < 0.0 {
                return Err(AgentDemoConfigError::Invalid(
                    "scripted_events[].at_seconds must be non-negative".into(),
                ));
            }
            match &event.action {
                ScriptedEventAction::RemoveFood { food_id } => {
                    if !food_ids.contains(food_id) {
                        return Err(AgentDemoConfigError::Invalid(format!(
                            "scripted event references unknown food id '{}'",
                            food_id
                        )));
                    }
                }
            }
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
pub struct ShowcaseConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorldConfig {
    pub floor_size: f32,
    #[serde(default = "default_floor_color")]
    pub floor_color: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CameraConfig {
    pub position: Vec3Config,
    pub look_at: Vec3Config,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub position: Vec3Config,
    pub perception_radius: f32,
    #[serde(default)]
    pub decision: DecisionConfig,
    pub motion: MotionConfig,
    pub hunger: HungerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MotionConfig {
    pub speed: f32,
    pub consume_distance: f32,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(default)]
pub struct DecisionConfig {
    pub hunger_weight: f32,
    pub nutrition_weight: f32,
    pub distance_weight: f32,
}

impl Default for DecisionConfig {
    fn default() -> Self {
        Self {
            hunger_weight: 1.0,
            nutrition_weight: 1.0,
            distance_weight: 0.5,
        }
    }
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
    pub id: String,
    pub position: Vec3Config,
    pub nutrition: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScriptedEventConfig {
    pub at_seconds: f32,
    #[serde(flatten)]
    pub action: ScriptedEventAction,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScriptedEventAction {
    RemoveFood { food_id: String },
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

pub fn parse_color_or(color: &str, fallback: Color) -> Color {
    match csscolorparser::parse(color) {
        Ok(parsed) => Color::srgba(parsed.r, parsed.g, parsed.b, parsed.a),
        Err(_) => fallback,
    }
}

fn default_floor_color() -> String {
    "#2d3338".to_string()
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

    const BASIC_SHOWCASE: &str = include_str!("../../assets/simulation/hungry_basic.toml");
    const CHOICE_SHOWCASE: &str = include_str!("../../assets/simulation/hungry_choice.toml");
    const REPLAN_SHOWCASE: &str = include_str!("../../assets/simulation/hungry_replan.toml");
    const PERCEPTION_SHOWCASE: &str =
        include_str!("../../assets/simulation/hungry_perception.toml");

    #[test]
    fn checked_in_basic_showcase_is_typed_and_valid() {
        let config = AgentDemoConfig::from_toml(BASIC_SHOWCASE)
            .expect("hungry_basic showcase config should be valid");

        assert_eq!(config.showcase.name, "hungry_basic");
        assert!(!config.foods.is_empty());
        assert!(config.agent.perception_radius > config.agent.motion.consume_distance);
    }

    #[test]
    fn all_checked_in_showcases_are_typed_and_valid() {
        for (name, source) in [
            ("hungry_basic", BASIC_SHOWCASE),
            ("hungry_choice", CHOICE_SHOWCASE),
            ("hungry_replan", REPLAN_SHOWCASE),
            ("hungry_perception", PERCEPTION_SHOWCASE),
        ] {
            let config = AgentDemoConfig::from_toml(source)
                .unwrap_or_else(|err| panic!("{name} showcase should be valid: {err}"));
            assert_eq!(config.showcase.name, name);
        }
    }

    #[test]
    fn rejects_impossible_hunger_values() {
        let source = r#"
            [showcase]
            name = "broken"
            description = "broken"

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
            perception_radius = 10.0
            [agent.position]
            x = 0.0
            y = 0.8
            z = 0.0
            [agent.motion]
            speed = 2.0
            consume_distance = 0.5
            [agent.hunger]
            initial = 200.0
            max = 100.0
            rate_per_second = 1.0
            seek_threshold = 50.0

            [[foods]]
            id = "apple"
            nutrition = 20.0
            [foods.position]
            x = 2.0
            y = 0.25
            z = 0.0
        "#;

        assert!(AgentDemoConfig::from_toml(source).is_err());
    }

    #[test]
    fn rejects_unknown_replan_event_target() {
        let source = r#"
            [showcase]
            name = "broken"
            description = "broken"

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
            perception_radius = 10.0
            [agent.position]
            x = 0.0
            y = 0.8
            z = 0.0
            [agent.motion]
            speed = 2.0
            consume_distance = 0.5
            [agent.hunger]
            initial = 20.0
            max = 100.0
            rate_per_second = 1.0
            seek_threshold = 50.0

            [[foods]]
            id = "apple"
            nutrition = 20.0
            [foods.position]
            x = 2.0
            y = 0.25
            z = 0.0

            [[scripted_events]]
            at_seconds = 1.5
            type = "remove_food"
            food_id = "missing"
        "#;

        assert!(AgentDemoConfig::from_toml(source).is_err());
    }
}
