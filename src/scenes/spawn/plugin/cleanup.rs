use bevy::prelude::*;
use bevy::time::{Time, Virtual};
use bevy_rapier3d::prelude::TimestepMode;

use crate::scenes::bounds::SceneBounds;
use crate::scenes::input::{
    ActionStates, CameraLookState, DebugMenuState, DebugMenuUiTag, DialogueState, DialogueUiTag,
    GrabHover, GrabState, NoclipState, PauseState, PlayerSpawn, SceneActionTriggers,
    SceneDialogueConfig, SceneFovConfig, SceneGrabConfig, SceneGrenadeConfig, SceneInputConfig,
    SceneJumpConfig, SceneNoclipConfig, ScenePauseConfig, SceneReloadConfig, SceneShootConfig,
    SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState,
};
use crate::scenes::spawn::{OverlayTag, SceneEntityTag};

pub(crate) fn cleanup_main_scene(
    mut commands: Commands,
    scene_entities: Query<Entity, With<SceneEntityTag>>,
    overlays: Query<Entity, With<OverlayTag>>,
    dialogue_ui: Query<Entity, With<DialogueUiTag>>,
    debug_menu_ui: Query<Entity, With<DebugMenuUiTag>>,
    time: Option<ResMut<Time<Virtual>>>,
    timestep: Option<ResMut<TimestepMode>>,
    pause_state: Option<Res<PauseState>>,
) {
    cleanup_main_scene_inner(
        &mut commands,
        &scene_entities,
        &overlays,
        &dialogue_ui,
        &debug_menu_ui,
        time,
        timestep,
        pause_state,
    );
}

pub(crate) fn cleanup_main_scene_inner(
    commands: &mut Commands,
    scene_entities: &Query<Entity, With<SceneEntityTag>>,
    overlays: &Query<Entity, With<OverlayTag>>,
    dialogue_ui: &Query<Entity, With<DialogueUiTag>>,
    debug_menu_ui: &Query<Entity, With<DebugMenuUiTag>>,
    mut time: Option<ResMut<Time<Virtual>>>,
    mut timestep: Option<ResMut<TimestepMode>>,
    pause_state: Option<Res<PauseState>>,
) {
    if let Some(pause_state) = pause_state {
        if pause_state.pause_scene {
            if let Some(mut time) = time.take() {
                time.unpause();
            }
            if let Some(mut timestep) = timestep.take() {
                if let TimestepMode::Variable { time_scale, .. } = &mut *timestep {
                    let restore = if pause_state.stored_time_scale > 0.0 {
                        pause_state.stored_time_scale
                    } else {
                        1.0
                    };
                    *time_scale = restore;
                }
            }
        }
    }

    for entity in scene_entities {
        commands.entity(entity).despawn();
    }
    for entity in overlays {
        commands.entity(entity).despawn();
    }
    for entity in dialogue_ui {
        commands.entity(entity).despawn();
    }
    for entity in debug_menu_ui {
        if let Ok(mut target) = commands.get_entity(entity) {
            target.despawn();
        }
    }

    commands.remove_resource::<SceneInputConfig>();
    commands.remove_resource::<SceneActionTriggers>();
    commands.remove_resource::<ActionStates>();
    commands.remove_resource::<SceneShootConfig>();
    commands.remove_resource::<SceneGrenadeConfig>();
    commands.remove_resource::<SceneSprintConfig>();
    commands.remove_resource::<SceneZoomConfig>();
    commands.remove_resource::<SceneJumpConfig>();
    commands.remove_resource::<SceneNoclipConfig>();
    commands.remove_resource::<SceneGrabConfig>();
    commands.remove_resource::<SceneReloadConfig>();
    commands.remove_resource::<ScenePauseConfig>();
    commands.remove_resource::<SceneDialogueConfig>();
    commands.remove_resource::<SceneFovConfig>();
    commands.remove_resource::<SprintState>();
    commands.remove_resource::<ZoomState>();
    commands.remove_resource::<NoclipState>();
    commands.remove_resource::<GrabState>();
    commands.remove_resource::<GrabHover>();
    commands.remove_resource::<PauseState>();
    commands.remove_resource::<DialogueState>();
    commands.remove_resource::<DebugMenuState>();
    commands.remove_resource::<PlayerSpawn>();
    commands.remove_resource::<CameraLookState>();
    commands.remove_resource::<SceneBounds>();
    commands.remove_resource::<AmbientLight>();
}
