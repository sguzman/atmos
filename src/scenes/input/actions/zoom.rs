use bevy::{
    input::keyboard::KeyCode,
    prelude::{ButtonInput, Projection, Query, Res, ResMut, With},
};

use super::super::types::{SceneCamera, SceneZoomConfig, ZoomState};

pub fn apply_zoom_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneZoomConfig>>,
    state: Option<ResMut<ZoomState>>,
    mut cameras: Query<&mut Projection, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(mut state) = state else {
        return;
    };

    let Ok(mut projection) = cameras.single_mut() else {
        return;
    };

    if state.base_fov.is_none() {
        if let Projection::Perspective(ref perspective) = *projection {
            state.base_fov = Some(perspective.fov);
        }
    }

    let Some(base_fov) = state.base_fov else {
        return;
    };

    let was_active = state.active;
    if config.action.toggle {
        if keys.just_pressed(config.trigger) {
            state.active = !state.active;
        }
    } else {
        state.active = keys.pressed(config.trigger);
    }

    if !was_active && state.active {
        if let Projection::Perspective(ref perspective) = *projection {
            state.base_fov = Some(perspective.fov);
        }
    }

    if state.active {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = config.action.fov_degrees.to_radians();
        }
    } else if was_active {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = base_fov;
        }
    }
}
