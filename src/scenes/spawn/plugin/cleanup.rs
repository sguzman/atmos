use bevy::prelude::*;

use crate::scenes::bounds::SceneBounds;
use crate::scenes::input::{
    CameraLookState, GrabHover, GrabState, NoclipState, PlayerSpawn, SceneFovConfig,
    SceneGrabConfig, SceneInputConfig, SceneJumpConfig, SceneNoclipConfig, SceneShootConfig,
    SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState,
};
use crate::scenes::spawn::{OverlayTag, SceneEntityTag};

pub(crate) fn cleanup_main_scene(
    mut commands: Commands,
    scene_entities: Query<Entity, With<SceneEntityTag>>,
    overlays: Query<Entity, With<OverlayTag>>,
) {
    for entity in &scene_entities {
        commands.entity(entity).despawn();
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<SceneInputConfig>();
    commands.remove_resource::<SceneShootConfig>();
    commands.remove_resource::<SceneSprintConfig>();
    commands.remove_resource::<SceneZoomConfig>();
    commands.remove_resource::<SceneJumpConfig>();
    commands.remove_resource::<SceneNoclipConfig>();
    commands.remove_resource::<SceneGrabConfig>();
    commands.remove_resource::<SceneFovConfig>();
    commands.remove_resource::<SprintState>();
    commands.remove_resource::<ZoomState>();
    commands.remove_resource::<NoclipState>();
    commands.remove_resource::<GrabState>();
    commands.remove_resource::<GrabHover>();
    commands.remove_resource::<PlayerSpawn>();
    commands.remove_resource::<CameraLookState>();
    commands.remove_resource::<SceneBounds>();
    commands.remove_resource::<AmbientLight>();
}
