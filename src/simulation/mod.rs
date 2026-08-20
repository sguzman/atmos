mod components;
mod config;
mod demo;
mod logic;
mod systems;

use std::path::PathBuf;

use bevy::prelude::*;

use components::SimulationClock;
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
        Self::new("assets/simulation/hungry_basic.toml")
    }
}

impl Plugin for AgentSimulationPlugin {
    fn build(&self, app: &mut App) {
        let config = AgentDemoConfig::load(&self.config_path).unwrap_or_else(|err| {
            panic!(
                "failed to load autonomous-agent showcase config {}: {err}",
                self.config_path.display()
            )
        });

        app.insert_resource(config)
            .init_resource::<SimulationClock>()
            .add_systems(Startup, demo::setup_demo)
            .add_systems(
                Update,
                (
                    systems::advance_simulation_clock,
                    systems::apply_scripted_events,
                    systems::update_agent_mind,
                    systems::move_agent_body,
                    systems::consume_selected_food,
                    systems::increase_agent_hunger,
                    systems::update_debug_text,
                )
                    .chain(),
            );
    }
}
