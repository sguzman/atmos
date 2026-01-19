use bevy::prelude::{Res, ResMut};

use super::super::types::{
    ActionStates, SceneSprintConfig,
    SprintState,
};

pub fn apply_sprint_toggle(
    config: Option<
        Res<SceneSprintConfig>,
    >,
    states: Option<Res<ActionStates>>,
    state: Option<ResMut<SprintState>>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut state) = state else {
        return;
    };
    if config.action.toggle
        && states
            .get(&config.id)
            .just_pressed
    {
        state.active = !state.active;
    }
}
