use bevy::{
    input::keyboard::KeyCode,
    prelude::{ButtonInput, Projection, Query, Res, ResMut, With},
};

use super::super::types::{SceneCamera, SceneFovConfig, ZoomState};

pub fn apply_fov_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneFovConfig>>,
    zoom_state: Option<ResMut<ZoomState>>,
    mut cameras: Query<&mut Projection, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

    let mut selected = None;
    for binding in &config.bindings {
        if keys.just_pressed(binding.trigger) {
            selected = Some(binding.fov_degrees);
        }
    }

    let Some(fov_degrees) = selected else {
        return;
    };

    let fov_radians = fov_degrees.to_radians();
    if let Some(mut zoom_state) = zoom_state {
        zoom_state.base_fov = Some(fov_radians);
        if zoom_state.active {
            return;
        }
    }

    for mut projection in cameras.iter_mut() {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = fov_radians;
        }
    }
}
