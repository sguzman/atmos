mod components;
mod config;
mod demo;
mod systems;

use std::path::PathBuf;

use bevy::prelude::*;

use config::AgentDemoConfig;

pub struct AgentSimulationPlugin {
    config_path: PathBuf,
}

impl AgentSimulationPlugin {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            config_path: path.into(),
        }
    }
}

impl Default for AgentSimulationPlugin {
    fn default() -> Self {
        Self::new("assets/simulation/agent_demo.toml")
    }
}

impl Plugin for AgentSimulationPlugin {
    fn build(&self, app: &mut App) {
        let config = AgentDemoConfig::load(&self.config_path).unwrap_or_else(|err| {
            panic!(
                "failed to load autonomous-agent demo config {}: {err}",
                self.config_path.display()
            )
        });

        app.insert_resource(config)
            .add_systems(Startup, demo::setup_demo)
            .add_systems(
                Update,
                (
                    systems::increase_hunger,
                    systems::choose_food_target,
                    systems::move_toward_food,
                    systems::consume_food,
                )
                    .chain(),
            );
    }
}
