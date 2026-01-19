use bevy::prelude::{
    Query, Res, ResMut, Visibility,
};
use bevy::time::{Time, Virtual};
use bevy_rapier3d::prelude::TimestepMode;

use crate::app_config::{
    AppConfig, AppMode,
};
use crate::scenes::input::{
    ActionStates, DebugMenuState,
    PauseState, ScenePauseConfig,
};
use crate::scenes::spawn::OverlayTag;

pub fn apply_pause_toggle(
    app_config: Res<AppConfig>,
    config: Option<
        Res<ScenePauseConfig>,
    >,
    states: Option<Res<ActionStates>>,
    pause_state: Option<
        ResMut<PauseState>,
    >,
    debug_menu: Option<
        Res<DebugMenuState>,
    >,
    mut overlays: Query<(
        &OverlayTag,
        &mut Visibility,
    )>,
    time: Option<ResMut<Time<Virtual>>>,
    timestep: Option<
        ResMut<TimestepMode>,
    >,
) {
    if matches!(
        app_config.mode,
        AppMode::Dev
    ) && app_config
        .debug_menu
        .enabled
        && debug_menu.is_some()
    {
        return;
    }
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut pause_state) =
        pause_state
    else {
        return;
    };

    if !states
        .get(&config.id)
        .just_pressed
    {
        return;
    }

    pause_state.active =
        !pause_state.active;

    for (_tag, mut vis) in overlays
        .iter_mut()
        .filter(|(tag, _)| {
            tag.name
                == pause_state.overlay
        })
    {
        *vis = if pause_state.active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !pause_state.pause_scene {
        return;
    }

    if pause_state.active {
        if let Some(mut time) = time {
            time.pause();
        }
        if let Some(mut timestep) =
            timestep
        {
            if let TimestepMode::Variable { time_scale, .. } = &mut *timestep {
                pause_state.stored_time_scale = *time_scale;
                *time_scale = 0.0;
            }
        }
    } else {
        if let Some(mut time) = time {
            time.unpause();
        }
        if let Some(mut timestep) =
            timestep
        {
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
