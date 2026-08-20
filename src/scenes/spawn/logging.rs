use bevy::{log::info, prelude::*};

#[derive(Resource, Default)]
pub(crate) struct SceneLogState {
    pub logged: bool,
}

pub(super) fn reset_scene_log_state(mut commands: Commands) {
    commands.insert_resource(SceneLogState::default());
}

pub(super) fn log_after_setup(
    setup: Option<Res<crate::scenes::spawn::plugin::SceneSetupState>>,
    mut state: ResMut<SceneLogState>,
    point_lights: Query<(&PointLight, &GlobalTransform, &Visibility, &ViewVisibility)>,
    dir_lights: Query<(
        &DirectionalLight,
        &GlobalTransform,
        &Visibility,
        &ViewVisibility,
    )>,
    ambient: Option<Res<AmbientLight>>,
    cameras: Query<(&Name, &Transform), With<Camera3d>>,
) {
    let Some(setup) = setup else {
        return;
    };
    if !setup.done || state.logged {
        return;
    }
    log_lights(point_lights, dir_lights, ambient);
    log_camera(cameras);
    state.logged = true;
}

pub(super) fn log_lights(
    point_lights: Query<(&PointLight, &GlobalTransform, &Visibility, &ViewVisibility)>,
    dir_lights: Query<(
        &DirectionalLight,
        &GlobalTransform,
        &Visibility,
        &ViewVisibility,
    )>,
    ambient: Option<Res<AmbientLight>>,
) {
    info!(
        "Lights present: {} point, {} directional, ambient: {:?}",
        point_lights.iter().len(),
        dir_lights.iter().len(),
        ambient.as_ref().map(|a| (a.color, a.brightness))
    );
    for (idx, (light, transform, vis, view_vis)) in point_lights.iter().enumerate() {
        info!(
            "Point light #{idx}: intensity={}, range={}, shadows={}, pos={:?}, visibility={:?}, view_visible={}",
            light.intensity,
            light.range,
            light.shadows_enabled,
            transform.translation(),
            vis,
            view_vis.get()
        );
    }
    for (idx, (light, transform, vis, view_vis)) in dir_lights.iter().enumerate() {
        info!(
            "Directional light #{idx}: illuminance={}, shadows={}, dir={:?}, visibility={:?}, view_visible={}",
            light.illuminance,
            light.shadows_enabled,
            transform.forward(),
            vis,
            view_vis.get()
        );
    }
}

pub(super) fn log_camera(cameras: Query<(&Name, &Transform), With<Camera3d>>) {
    for (name, transform) in cameras.iter() {
        info!(
            "Camera '{}' at {:?} looking {:?}",
            name,
            transform.translation,
            transform.forward()
        );
    }
}
