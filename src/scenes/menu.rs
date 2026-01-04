use bevy::{
    app::AppExit,
    input::keyboard::KeyCode,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy::state::state::NextState;
use bevy::state::condition::in_state;

use super::input::{resolve_camera_input_config, resolve_overlay_toggles, SceneInputConfig};
use super::loaders::load_input_config;
use super::spawn::{spawn_overlays_from_config, OverlayTag};
use super::AppState;

#[derive(Component)]
struct MenuCamera;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu);
        app.add_systems(
            OnEnter(AppState::Menu),
            spawn_overlays_from_config.after(setup_menu),
        );
        app.add_systems(OnExit(AppState::Menu), cleanup_menu);
        app.add_systems(Update, handle_menu_input.run_if(in_state(AppState::Menu)));
        app.add_systems(OnEnter(AppState::Menu), configure_menu_cursor);
    }
}

fn setup_menu(mut commands: Commands) {
    let input_config = load_input_config("menu");
    let camera_input =
        resolve_camera_input_config(&input_config.camera.movement, &input_config.camera.rotation);
    commands.insert_resource(SceneInputConfig {
        camera: camera_input,
        overlays: resolve_overlay_toggles(&input_config.overlays),
    });

    commands.spawn((Camera2d::default(), MenuCamera));
}

fn cleanup_menu(
    mut commands: Commands,
    overlays: Query<(Entity, &OverlayTag)>,
    cameras: Query<Entity, With<MenuCamera>>,
) {
    for (entity, tag) in &overlays {
        if tag.name == "menu" {
            commands.entity(entity).despawn();
        }
    }
    for entity in &cameras {
        commands.entity(entity).despawn();
    }
}

fn handle_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        next_state.set(AppState::Main);
    }
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyQ) {
        app_exit.write(AppExit::Success);
    }
}

fn configure_menu_cursor(mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = windows.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        cursor.hit_test = true;
    }
}
