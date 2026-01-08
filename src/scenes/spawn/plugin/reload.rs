use bevy::prelude::{Commands, Entity, Query, Res, ResMut, With};
use bevy::time::{Time, Virtual};
use bevy_rapier3d::prelude::TimestepMode;

use crate::scenes::input::{ActionStates, PauseState, SceneReloadConfig};
use crate::scenes::spawn::{OverlayTag, SceneEntityTag};

use super::cleanup::cleanup_main_scene_inner;
use super::SceneSetupState;

pub(crate) fn apply_scene_reload(
    config: Option<Res<SceneReloadConfig>>,
    states: Option<Res<ActionStates>>,
    mut commands: Commands,
    scene_entities: Query<Entity, With<SceneEntityTag>>,
    overlays: Query<Entity, With<OverlayTag>>,
    time: Option<ResMut<Time<Virtual>>>,
    timestep: Option<ResMut<TimestepMode>>,
    pause_state: Option<Res<PauseState>>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    if !states.get(&config.id).just_pressed {
        return;
    }

    cleanup_main_scene_inner(
        &mut commands,
        &scene_entities,
        &overlays,
        time,
        timestep,
        pause_state,
    );
    commands.insert_resource(SceneSetupState::default());
    commands.insert_resource(super::super::overlay::OverlaySpawnState::default());
    commands.insert_resource(super::super::logging::SceneLogState::default());
}
