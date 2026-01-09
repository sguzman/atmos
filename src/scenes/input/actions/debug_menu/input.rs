use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::ui::ComputedNode;
use bevy::window::{PrimaryWindow, Window};
use bevy_rapier3d::prelude::{DefaultRapierContext, RapierConfiguration};

use crate::app_config::AppConfig;
use crate::scenes::input::{DebugMenuState, SceneCamera, ZoomState};
use crate::scenes::spawn::SunLight;

use super::actions::{apply_debug_menu_action, apply_slider_value};
use super::state::{debug_menu_enabled, safe_despawn};
use super::types::{DebugMenuButton, DebugMenuSlider, DebugMenuSliderLabel, DebugMenuUiTag};
use super::ui::{spawn_debug_menu_ui, update_slider_label};

pub fn update_debug_menu_ui(
    app_config: Res<AppConfig>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    debug_state: Option<ResMut<DebugMenuState>>,
    mut commands: Commands,
    ui_nodes: Query<Entity, With<DebugMenuUiTag>>,
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &Children, &DebugMenuButton), Changed<Interaction>>,
    mut slider_params: ParamSet<(
        Query<(Entity, &Interaction, &DebugMenuSlider, &ComputedNode, &GlobalTransform)>,
        Query<&mut Node>,
        Query<(&mut Text, &DebugMenuSliderLabel)>,
    )>,
    mut text_colors: Query<&mut TextColor>,
    mut projections: Query<&mut Projection, With<SceneCamera>>,
    mut zoom_state: Option<ResMut<ZoomState>>,
    camera_entities: Query<Entity, With<SceneCamera>>,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    mut sun: Query<&mut DirectionalLight, With<SunLight>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if !debug_menu_enabled(&app_config) {
        return;
    }
    let Some(mut debug_state) = debug_state else {
        return;
    };
    if !debug_state.active {
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Right) {
        if debug_state.stack.len() > 1 {
            debug_state.stack.pop();
            debug_state.needs_refresh = true;
        }
    }

    for (interaction, mut background, children, button) in buttons.iter_mut() {
        let (bg_color, text_color) = match *interaction {
            Interaction::Pressed => (Color::srgba(0.35, 0.35, 0.35, 0.95), Color::BLACK),
            Interaction::Hovered => (Color::srgba(0.25, 0.25, 0.25, 0.9), Color::WHITE),
            Interaction::None => (Color::srgba(0.1, 0.1, 0.1, 0.85), Color::WHITE),
        };
        *background = bg_color.into();
        for &child in children {
            if let Ok(mut color) = text_colors.get_mut(child) {
                color.0 = text_color;
            }
        }

        if *interaction == Interaction::Pressed {
            apply_debug_menu_action(
                &button.action,
                &mut debug_state,
                &mut commands,
                &camera_entities,
                &mut rapier_config,
                &mut sun,
            );
        }
    }

    handle_slider_input(
        &mouse_buttons,
        &windows,
        &mut debug_state,
        &mut slider_params,
        &mut commands,
        &mut projections,
        &mut zoom_state,
        &camera_entities,
        &mut rapier_config,
        &mut sun,
    );

    if debug_state.needs_refresh {
        for entity in &ui_nodes {
            safe_despawn(&mut commands, entity);
        }
        spawn_debug_menu_ui(&mut commands, &asset_server, &debug_state);
        debug_state.needs_refresh = false;
    }
}

fn handle_slider_input(
    mouse_buttons: &ButtonInput<MouseButton>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    debug_state: &mut DebugMenuState,
    slider_params: &mut ParamSet<(
        Query<(Entity, &Interaction, &DebugMenuSlider, &ComputedNode, &GlobalTransform)>,
        Query<&mut Node>,
        Query<(&mut Text, &DebugMenuSliderLabel)>,
    )>,
    commands: &mut Commands,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
    camera_entities: &Query<Entity, With<SceneCamera>>,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    if mouse_buttons.just_released(MouseButton::Left) {
        debug_state.active_slider = None;
    }

    let mut updates = Vec::new();
    {
        let mut sliders = slider_params.p0();
        for (entity, interaction, slider, computed, transform) in sliders.iter_mut() {
            if mouse_buttons.just_pressed(MouseButton::Left) && *interaction == Interaction::Hovered {
                debug_state.active_slider = Some(entity);
            }

            if debug_state.active_slider != Some(entity) {
                continue;
            }

            let size = computed.size;
            let center = transform.translation().truncate();
            let left = center.x - size.x / 2.0;
            let ratio = ((cursor.x - left) / size.x).clamp(0.0, 1.0);
            let value = slider.min + (slider.max - slider.min) * ratio;

            apply_slider_value(
                slider.kind,
                value,
                debug_state,
                commands,
                projections,
                zoom_state,
                camera_entities,
                rapier_config,
                sun,
            );
            updates.push((slider.fill, slider.kind, ratio, value));
        }
    }

    if updates.is_empty() {
        return;
    }

    {
        let mut slider_fills = slider_params.p1();
        for (fill_entity, _kind, ratio, _value) in &updates {
            if let Ok(mut fill) = slider_fills.get_mut(*fill_entity) {
                fill.width = Val::Percent(ratio * 100.0);
            }
        }
    }

    {
        let mut slider_labels = slider_params.p2();
        for (_fill_entity, kind, _ratio, value) in updates {
            update_slider_label(&mut slider_labels, kind, value);
        }
    }
}
