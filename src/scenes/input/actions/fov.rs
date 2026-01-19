use bevy::prelude::{
    Projection, Query, Res, ResMut,
    With,
};

use super::super::types::{
    ActionStates, SceneCamera,
    SceneFovConfig, ZoomState,
};

pub fn apply_fov_action(
    config: Option<Res<SceneFovConfig>>,
    states: Option<Res<ActionStates>>,
    zoom_state: Option<
        ResMut<ZoomState>,
    >,
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

    let mut selected = None;
    for binding in &config.bindings {
        if states
            .get(&binding.action_id)
            .just_pressed
        {
            selected = Some(
                binding.fov_degrees,
            );
        }
    }

    let Some(fov_degrees) = selected
    else {
        return;
    };

    let fov_radians =
        fov_degrees.to_radians();
    if let Some(mut zoom_state) =
        zoom_state
    {
        zoom_state.base_fov =
            Some(fov_radians);
        if zoom_state.active {
            return;
        }
    }

    for mut projection in
        cameras.iter_mut()
    {
        if let Projection::Perspective(
            ref mut perspective,
        ) = *projection
        {
            perspective.fov =
                fov_radians;
        }
    }
}
