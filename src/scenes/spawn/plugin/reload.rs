use bevy::{
    input::keyboard::KeyCode,
    prelude::{ButtonInput, Commands, Entity, Query, Res, With},
};

use crate::scenes::input::SceneReloadConfig;
use crate::scenes::spawn::{OverlayTag, SceneEntityTag};

use super::cleanup::cleanup_main_scene_inner;
use super::SceneSetupState;

pub(crate) fn apply_scene_reload(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneReloadConfig>>,
    mut commands: Commands,
    scene_entities: Query<Entity, With<SceneEntityTag>>,
    overlays: Query<Entity, With<OverlayTag>>,
) {
    let Some(config) = config else {
        return;
    };
    if !keys.just_pressed(config.trigger) {
        return;
    }

    cleanup_main_scene_inner(&mut commands, &scene_entities, &overlays);
    commands.insert_resource(SceneSetupState::default());
    commands.insert_resource(super::super::overlay::OverlaySpawnState::default());
    commands.insert_resource(super::super::logging::SceneLogState::default());
}
