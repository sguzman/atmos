use bevy::ecs::system::entity_command;
use bevy::pbr::{
    DistanceFog, FogFalloff,
};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::time::{Time, Virtual};
use bevy::window::{
    CursorGrabMode, CursorOptions,
    PrimaryWindow,
};
use bevy_rapier3d::prelude::{
    DefaultRapierContext,
    RapierConfiguration, TimestepMode,
};

use crate::app_config::{
    AppConfig, AppMode,
};
use crate::scenes::input::{
    ActionStates, DebugMenuPage,
    DebugMenuState, SceneCamera,
    ScenePauseConfig,
};
use crate::scenes::spawn::SunLight;

use super::types::DebugMenuUiTag;

pub(crate) fn debug_menu_enabled(
    app_config: &AppConfig,
) -> bool {
    matches!(
        app_config.mode,
        AppMode::Dev
    ) && app_config.debug_menu.enabled
}

pub fn apply_debug_menu_toggle(
    app_config: Res<AppConfig>,
    config: Option<
        Res<ScenePauseConfig>,
    >,
    states: Option<Res<ActionStates>>,
    debug_state: Option<
        ResMut<DebugMenuState>,
    >,
    mut commands: Commands,
    ui_nodes: Query<
        Entity,
        With<DebugMenuUiTag>,
    >,
    mut windows: Query<
        &mut CursorOptions,
        With<PrimaryWindow>,
    >,
    time: Option<ResMut<Time<Virtual>>>,
    timestep: Option<
        ResMut<TimestepMode>,
    >,
    cameras: Query<
        (
            Entity,
            &Projection,
            Option<&Bloom>,
            Option<&DistanceFog>,
        ),
        With<SceneCamera>,
    >,
    rapier_config: Query<
        &RapierConfiguration,
        With<DefaultRapierContext>,
    >,
    sun: Query<
        &DirectionalLight,
        With<SunLight>,
    >,
) {
    if !debug_menu_enabled(&app_config)
    {
        return;
    }
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut debug_state) =
        debug_state
    else {
        return;
    };

    if !states
        .get(&config.id)
        .just_pressed
    {
        return;
    }

    if debug_state.active {
        close_debug_menu(
            &app_config,
            &mut debug_state,
            &mut commands,
            &ui_nodes,
            &mut windows,
            time,
            timestep,
        );
        return;
    }

    open_debug_menu(
        &app_config,
        &mut debug_state,
        &mut windows,
        time,
        timestep,
        &cameras,
        &rapier_config,
        &sun,
    );
}

fn open_debug_menu(
    app_config: &AppConfig,
    debug_state: &mut DebugMenuState,
    windows: &mut Query<
        &mut CursorOptions,
        With<PrimaryWindow>,
    >,
    mut time: Option<
        ResMut<Time<Virtual>>,
    >,
    mut timestep: Option<
        ResMut<TimestepMode>,
    >,
    cameras: &Query<
        (
            Entity,
            &Projection,
            Option<&Bloom>,
            Option<&DistanceFog>,
        ),
        With<SceneCamera>,
    >,
    rapier_config: &Query<
        &RapierConfiguration,
        With<DefaultRapierContext>,
    >,
    sun: &Query<
        &DirectionalLight,
        With<SunLight>,
    >,
) {
    debug_state.active = true;
    if debug_state.stack.is_empty() {
        debug_state
            .stack
            .push(DebugMenuPage::Root);
    }
    debug_state.needs_refresh = true;

    initialize_debug_menu_settings(
        debug_state,
        cameras,
        rapier_config,
        sun,
    );

    if let Ok(mut cursor) =
        windows.single_mut()
    {
        cursor.grab_mode =
            CursorGrabMode::None;
        cursor.visible = true;
    }

    if app_config.debug_menu.pause_scene
    {
        if let Some(mut time) =
            time.take()
        {
            time.pause();
        }
        if let Some(mut timestep) =
            timestep.take()
        {
            if let TimestepMode::Variable { time_scale, .. } = &mut *timestep {
                debug_state.stored_time_scale = *time_scale;
                *time_scale = 0.0;
            }
        }
    }

    debug_state.active_slider = None;
}

fn close_debug_menu(
    app_config: &AppConfig,
    debug_state: &mut DebugMenuState,
    commands: &mut Commands,
    ui_nodes: &Query<
        Entity,
        With<DebugMenuUiTag>,
    >,
    windows: &mut Query<
        &mut CursorOptions,
        With<PrimaryWindow>,
    >,
    mut time: Option<
        ResMut<Time<Virtual>>,
    >,
    mut timestep: Option<
        ResMut<TimestepMode>,
    >,
) {
    debug_state.active = false;
    debug_state.needs_refresh = false;

    for entity in ui_nodes {
        safe_despawn(commands, entity);
    }

    if let Ok(mut cursor) =
        windows.single_mut()
    {
        cursor.grab_mode =
            CursorGrabMode::Locked;
        cursor.visible = false;
    }

    if app_config.debug_menu.pause_scene
    {
        if let Some(mut time) =
            time.take()
        {
            time.unpause();
        }
        if let Some(mut timestep) =
            timestep.take()
        {
            if let TimestepMode::Variable { time_scale, .. } = &mut *timestep {
                let restore = if debug_state.stored_time_scale > 0.0 {
                    debug_state.stored_time_scale
                } else {
                    1.0
                };
                *time_scale = restore;
            }
        }
    }
}

fn initialize_debug_menu_settings(
    debug_state: &mut DebugMenuState,
    cameras: &Query<
        (
            Entity,
            &Projection,
            Option<&Bloom>,
            Option<&DistanceFog>,
        ),
        With<SceneCamera>,
    >,
    rapier_config: &Query<
        &RapierConfiguration,
        With<DefaultRapierContext>,
    >,
    sun: &Query<
        &DirectionalLight,
        With<SunLight>,
    >,
) {
    if debug_state.settings.initialized
    {
        return;
    }

    if let Ok((
        _entity,
        projection,
        bloom,
        fog,
    )) = cameras.single()
    {
        if let Projection::Perspective(
            perspective,
        ) = projection
        {
            debug_state
                .settings
                .fov_degrees =
                perspective
                    .fov
                    .to_degrees();
        }
        debug_state
            .settings
            .bloom_enabled =
            bloom.is_some();
        debug_state.settings.bloom =
            bloom.cloned();
        if let Some(bloom) = bloom {
            debug_state
                .settings
                .bloom_intensity =
                bloom.intensity;
            debug_state
                .settings
                .bloom_threshold =
                bloom
                    .prefilter
                    .threshold;
            debug_state.settings.bloom_threshold_softness = bloom.prefilter.threshold_softness;
        }
        debug_state
            .settings
            .fog_enabled =
            fog.is_some();
        debug_state.settings.fog =
            fog.cloned();
        if let Some(fog) = fog {
            debug_state
                .settings
                .fog_alpha =
                fog.color.alpha();
            match &fog.falloff {
                FogFalloff::Linear { start, end } => {
                    debug_state.settings.fog_mode = "linear".to_string();
                    debug_state.settings.fog_linear_start = *start;
                    debug_state.settings.fog_linear_end = *end;
                }
                FogFalloff::Exponential { density } => {
                    debug_state.settings.fog_mode = "exponential".to_string();
                    debug_state.settings.fog_density = *density;
                }
                FogFalloff::ExponentialSquared { density } => {
                    debug_state.settings.fog_mode = "exponential_squared".to_string();
                    debug_state.settings.fog_density = *density;
                }
                FogFalloff::Atmospheric { .. } => {
                    debug_state.settings.fog_mode = "linear".to_string();
                }
            }
        }
    }

    if let Ok(config) =
        rapier_config.single()
    {
        debug_state.settings.gravity =
            Vec3::new(
                config.gravity.x,
                config.gravity.y,
                config.gravity.z,
            );
        debug_state
            .settings
            .physics_enabled = config
            .physics_pipeline_active;
    }

    if let Ok(light) = sun.single() {
        debug_state
            .settings
            .sun_present = true;
        debug_state
            .settings
            .sun_brightness =
            light.illuminance;
        debug_state
            .settings
            .sun_shadows =
            light.shadows_enabled;
    }

    debug_state.settings.initialized =
        true;
}

pub(crate) fn safe_despawn(
    commands: &mut Commands,
    entity: Entity,
) {
    commands
        .entity(entity)
        .queue_silenced(
            entity_command::despawn(),
        );
}
