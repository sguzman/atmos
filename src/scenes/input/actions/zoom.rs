use bevy::prelude::{
    Projection, Query, Res, ResMut,
    With,
};

use super::super::types::{
    ActionStates, SceneCamera,
    SceneZoomConfig, ZoomState,
};

pub fn apply_zoom_action(
    config: Option<
        Res<SceneZoomConfig>,
    >,
    states: Option<Res<ActionStates>>,
    state: Option<ResMut<ZoomState>>,
    mut cameras: Query<
        &mut Projection,
        With<SceneCamera>,
    >,
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

    let Ok(mut projection) =
        cameras.single_mut()
    else {
        return;
    };

    if state.base_fov.is_none() {
        if let Projection::Perspective(
            ref perspective,
        ) = *projection
        {
            state.base_fov =
                Some(perspective.fov);
        }
    }

    let Some(base_fov) = state.base_fov
    else {
        return;
    };

    let was_active = state.active;
    if config.action.toggle {
        if states
            .get(&config.id)
            .just_pressed
        {
            state.active =
                !state.active;
        }
    } else {
        state.active = states
            .get(&config.id)
            .pressed;
    }

    if !was_active && state.active {
        if let Projection::Perspective(
            ref perspective,
        ) = *projection
        {
            state.base_fov =
                Some(perspective.fov);
        }
    }

    if state.active {
        if let Projection::Perspective(
            ref mut perspective,
        ) = *projection
        {
            perspective.fov = config
                .action
                .fov_degrees
                .to_radians();
        }
    } else if was_active {
        if let Projection::Perspective(
            ref mut perspective,
        ) = *projection
        {
            perspective.fov = base_fov;
        }
    }
}
