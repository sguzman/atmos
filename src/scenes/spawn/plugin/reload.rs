use bevy::prelude::{Commands, Entity, Query, Res, With};

use crate::scenes::input::{ActionStates, SceneReloadConfig};
use crate::scenes::spawn::{OverlayTag, SceneEntityTag};

use super::cleanup::cleanup_main_scene_inner;
use super::SceneSetupState;

pub(crate) fn apply_scene_reload(
    config: Option<Res<SceneReloadConfig>>,
    states: Option<Res<ActionStates>>,
    mut commands: Commands,
    scene_entities: Query<Entity, With<SceneEntityTag>>,
    overlays: Query<Entity, With<OverlayTag>>,
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

    cleanup_main_scene_inner(&mut commands, &scene_entities, &overlays);
    commands.insert_resource(SceneSetupState::default());
    commands.insert_resource(super::super::overlay::OverlaySpawnState::default());
    commands.insert_resource(super::super::logging::SceneLogState::default());
}
