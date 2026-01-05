use bevy::{
    app::AppExit,
    input::keyboard::KeyCode,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy::state::state::NextState;
use bevy::state::condition::in_state;

use super::input::{resolve_camera_input_config, resolve_key_or_warn, resolve_overlay_toggles, SceneInputConfig};
use super::loaders::{
    load_input_config, load_quit_action_config, load_scene_transition_action_config, ConfigLoad,
    TomlCache,
};
use super::spawn::{spawn_overlays_from_config, reset_overlay_spawn_state, OverlayTag};
use super::TomlAsset;
use super::AppState;

#[derive(Component)]
struct MenuCamera;

#[derive(Resource)]
struct MenuSceneTransition {
    trigger: KeyCode,
    target_scene: String,
}

#[derive(Resource)]
struct MenuQuitAction {
    trigger: KeyCode,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSetupState>();
        app.add_systems(OnEnter(AppState::Menu), reset_menu_setup_state);
        app.add_systems(OnEnter(AppState::Menu), reset_overlay_spawn_state);
        app.add_systems(OnEnter(AppState::Menu), configure_menu_cursor);
        app.add_systems(Update, setup_menu.run_if(in_state(AppState::Menu)));
        app.add_systems(
            Update,
            spawn_overlays_from_config.run_if(in_state(AppState::Menu)),
        );
        app.add_systems(OnExit(AppState::Menu), cleanup_menu);
        app.add_systems(Update, handle_menu_input.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Resource, Default)]
struct MenuSetupState {
    done: bool,
}

fn reset_menu_setup_state(mut commands: Commands) {
    commands.insert_resource(MenuSetupState::default());
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    toml_assets: Res<Assets<TomlAsset>>,
    mut toml_cache: ResMut<TomlCache>,
    mut setup_state: ResMut<MenuSetupState>,
) {
    if setup_state.done {
        return;
    }
    let input_config = match load_input_config("menu", &mut toml_cache, &asset_server, &toml_assets)
    {
        ConfigLoad::Pending => return,
        ConfigLoad::Ready(config) => config,
    };
    let camera_input =
        resolve_camera_input_config(&input_config.camera.movement, &input_config.camera.rotation);
    commands.insert_resource(SceneInputConfig {
        camera: camera_input,
        overlays: resolve_overlay_toggles(&input_config.overlays),
    });

    let mut transition = None;
    let mut quit = None;
    for action_binding in &input_config.actions {
        if action_binding.action.ends_with("scene-transition.toml") {
            if let Some(trigger) =
                resolve_key_or_warn(&action_binding.key, "menu scene transition")
            {
                let action = match load_scene_transition_action_config(
                    "menu",
                    &action_binding.action,
                    &mut toml_cache,
                    &asset_server,
                    &toml_assets,
                ) {
                    ConfigLoad::Pending => return,
                    ConfigLoad::Ready(action) => action,
                };
                if let Some(action) = action {
                    transition = Some(MenuSceneTransition {
                        trigger,
                        target_scene: action.target_scene,
                    });
                }
            }
        }
        if action_binding.action.ends_with("quit.toml") {
            if let Some(trigger) = resolve_key_or_warn(&action_binding.key, "menu quit") {
                let action = match load_quit_action_config(
                    "menu",
                    &action_binding.action,
                    &mut toml_cache,
                    &asset_server,
                    &toml_assets,
                ) {
                    ConfigLoad::Pending => return,
                    ConfigLoad::Ready(action) => action,
                };
                if action.is_some() {
                    quit = Some(MenuQuitAction { trigger });
                }
            }
        }
    }

    if let Some(transition) = transition {
        commands.insert_resource(transition);
    }
    if let Some(quit) = quit {
        commands.insert_resource(quit);
    }

    let use_2d = input_config
        .camera
        .mode
        .trim()
        .eq_ignore_ascii_case("2d");
    if use_2d {
        commands.spawn((Camera2d::default(), MenuCamera));
    } else {
        commands.spawn((Camera3d::default(), MenuCamera));
    }

    setup_state.done = true;
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
    transition: Option<Res<MenuSceneTransition>>,
    quit: Option<Res<MenuQuitAction>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if let Some(transition) = transition.as_ref() {
        if keys.just_pressed(transition.trigger) {
            next_state.set(AppState::from_scene_name(&transition.target_scene));
        }
    }
    if let Some(quit) = quit.as_ref() {
        if keys.just_pressed(quit.trigger) {
            app_exit.write(AppExit::Success);
        }
    }
}

fn configure_menu_cursor(mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = windows.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        cursor.hit_test = true;
    }
}
