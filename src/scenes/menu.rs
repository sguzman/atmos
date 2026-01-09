use bevy::{
    app::AppExit,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy::state::state::NextState;
use bevy::state::condition::in_state;

use super::config::{
    ActionConfig, ActionTriggerConfig, ActionsConfig, TriggerMode, VolumeShapeKind,
    VolumeTriggerMode,
};
use super::input::{
    resolve_camera_input_config, resolve_key_or_warn, resolve_mouse_button_or_warn,
    resolve_overlay_toggles, ActionStates, ResolvedActionTrigger, ResolvedVolumeTrigger,
    SceneActionTriggers, SceneInputConfig, TriggerMode as InputTriggerMode, TriggerSource,
    VolumeShape, VolumeShapeKind as InputVolumeShapeKind, VolumeTriggerMode as InputVolumeTriggerMode,
};
use super::loaders::{
    load_actions_config, load_input_config, ConfigLoad, TomlCache,
};
use super::spawn::{spawn_overlays_from_config, reset_overlay_spawn_state, OverlayTag};
use super::TomlAsset;
use super::AppState;

#[derive(Component)]
struct MenuCamera;

#[derive(Resource)]
struct MenuSceneTransition {
    action_id: String,
    target_scene: String,
}

#[derive(Resource)]
struct MenuQuitAction {
    action_id: String,
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
        app.add_systems(Update, super::input::update_action_states.run_if(in_state(AppState::Menu)));
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

    let actions_config: ActionsConfig = match load_actions_config(
        "menu",
        &mut toml_cache,
        &asset_server,
        &toml_assets,
    ) {
        ConfigLoad::Pending => return,
        ConfigLoad::Ready(config) => config,
    };

    let mut transition = None;
    let mut quit = None;
    for action in actions_config.actions.iter() {
        match action {
            ActionConfig::SceneTransition { id, params } => {
                transition = Some(MenuSceneTransition {
                    action_id: id.clone(),
                    target_scene: params.target_scene.clone(),
                });
            }
            ActionConfig::Quit { id, .. } => {
                quit = Some(MenuQuitAction {
                    action_id: id.clone(),
                });
            }
            _ => {}
        }
    }

    if let Some(transition) = transition {
        commands.insert_resource(transition);
    }
    if let Some(quit) = quit {
        commands.insert_resource(quit);
    }

    let mut resolved_triggers = Vec::new();
    let mut resolved_volumes = Vec::new();
    for trigger in actions_config.triggers.iter() {
        match trigger {
            ActionTriggerConfig::Key { key, mode, action, .. } => {
                if let Some(trigger) = resolve_key_or_warn(key, "menu action key") {
                    resolved_triggers.push(ResolvedActionTrigger {
                        action: action.clone(),
                        source: TriggerSource::Key(trigger),
                        mode: map_trigger_mode(*mode),
                    });
                }
            }
            ActionTriggerConfig::Mouse { mouse, mode, action, .. } => {
                if let Some(trigger) = resolve_mouse_button_or_warn(mouse, "menu action mouse") {
                    resolved_triggers.push(ResolvedActionTrigger {
                        action: action.clone(),
                        source: TriggerSource::Mouse(trigger),
                        mode: map_trigger_mode(*mode),
                    });
                }
            }
            ActionTriggerConfig::Volume {
                action,
                mode,
                shape,
                transform,
                once,
                ..
            } => {
                let (kind, radius, size) = match shape.kind {
                    VolumeShapeKind::Sphere => (
                        InputVolumeShapeKind::Sphere,
                        shape.radius.unwrap_or(1.0).max(0.0),
                        Vec3::ZERO,
                    ),
                    VolumeShapeKind::Box => {
                        let size_cfg = shape.size.clone().unwrap_or_default();
                        (
                            InputVolumeShapeKind::Box,
                            0.0,
                            Vec3::new(size_cfg.width, size_cfg.height, size_cfg.depth),
                        )
                    }
                };
                resolved_volumes.push(ResolvedVolumeTrigger {
                    action: action.clone(),
                    mode: map_volume_mode(*mode),
                    shape: VolumeShape { kind, radius, size },
                    position: Vec3::new(transform.x, transform.y, transform.z),
                    once: *once,
                    fired: false,
                    inside: false,
                });
            }
        }
    }
    commands.insert_resource(SceneActionTriggers {
        input: resolved_triggers,
        volumes: resolved_volumes,
    });
    commands.insert_resource(ActionStates::default());

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
    for (entity, _tag) in &overlays {
        commands
            .entity(entity)
            .queue_silenced(bevy::ecs::system::entity_command::despawn());
    }
    for entity in &cameras {
        commands
            .entity(entity)
            .queue_silenced(bevy::ecs::system::entity_command::despawn());
    }
    commands.remove_resource::<SceneInputConfig>();
    commands.remove_resource::<MenuSceneTransition>();
    commands.remove_resource::<MenuQuitAction>();
    commands.remove_resource::<SceneActionTriggers>();
    commands.remove_resource::<ActionStates>();
}

fn handle_menu_input(
    transition: Option<Res<MenuSceneTransition>>,
    quit: Option<Res<MenuQuitAction>>,
    states: Option<Res<ActionStates>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let Some(states) = states else {
        return;
    };
    if let Some(transition) = transition.as_ref() {
        if states.get(&transition.action_id).just_pressed {
            next_state.set(AppState::from_scene_name(&transition.target_scene));
        }
    }
    if let Some(quit) = quit.as_ref() {
        if states.get(&quit.action_id).just_pressed {
            app_exit.write(AppExit::Success);
        }
    }
}

fn map_trigger_mode(mode: TriggerMode) -> InputTriggerMode {
    match mode {
        TriggerMode::Press => InputTriggerMode::Press,
        TriggerMode::Hold => InputTriggerMode::Hold,
    }
}

fn map_volume_mode(mode: VolumeTriggerMode) -> InputVolumeTriggerMode {
    match mode {
        VolumeTriggerMode::Enter => InputVolumeTriggerMode::Enter,
        VolumeTriggerMode::Exit => InputVolumeTriggerMode::Exit,
        VolumeTriggerMode::Inside => InputVolumeTriggerMode::Inside,
    }
}

fn configure_menu_cursor(mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = windows.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        cursor.hit_test = true;
    }
}
