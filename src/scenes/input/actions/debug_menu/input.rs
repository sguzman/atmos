use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{DefaultRapierContext, RapierConfiguration};

use crate::app_config::AppConfig;
use crate::scenes::input::{DebugMenuState, SceneCamera, ZoomState};
use crate::scenes::spawn::SunLight;

use super::actions::apply_debug_menu_action;
use super::state::{debug_menu_enabled, safe_despawn};
use super::types::{DebugMenuButton, DebugMenuUiTag};
use super::ui::spawn_debug_menu_ui;

pub fn update_debug_menu_ui(
    app_config: Res<AppConfig>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    debug_state: Option<ResMut<DebugMenuState>>,
    mut commands: Commands,
    ui_nodes: Query<Entity, With<DebugMenuUiTag>>,
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &DebugMenuButton,
        ),
        Changed<Interaction>,
    >,
    mut text_colors: Query<&mut TextColor>,
    mut projections: Query<&mut Projection, With<SceneCamera>>,
    mut zoom_state: Option<ResMut<ZoomState>>,
    camera_entities: Query<Entity, With<SceneCamera>>,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    mut sun: Query<&mut DirectionalLight, With<SunLight>>,
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
                &mut projections,
                &mut zoom_state,
                &camera_entities,
                &mut rapier_config,
                &mut sun,
            );
        }
    }

    if debug_state.needs_refresh {
        for entity in &ui_nodes {
            safe_despawn(&mut commands, entity);
        }
        spawn_debug_menu_ui(&mut commands, &asset_server, &debug_state);
        debug_state.needs_refresh = false;
    }
}
