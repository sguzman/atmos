use bevy::{input::keyboard::KeyCode, prelude::{ButtonInput, Res, ResMut}};

use super::super::types::{SceneSprintConfig, SprintState};

pub fn apply_sprint_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneSprintConfig>>,
    mut state: ResMut<SprintState>,
) {
    let Some(config) = config else {
        return;
    };
    if config.action.toggle && keys.just_pressed(config.trigger) {
        state.active = !state.active;
    }
}
