mod app_config;
mod scenes;

use app_config::load_app_config;
use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use bevy_rapier3d::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    let app_config = load_app_config();
    let log_plugin = app_config.to_log_plugin();
    let window_plugin = app_config.to_window_plugin();

    let mut app = App::new();

    app.insert_resource::<WinitSettings>(app_config.winit_settings());
    app.insert_resource(app_config.clone());

    let world_config = scenes::load_world_config("main");
    let startup_scene = world_config
        .startup_scene
        .as_deref()
        .unwrap_or("main");
    let initial_state = scenes::AppState::from_scene_name(startup_scene);

    app.add_plugins(
        DefaultPlugins
            .set(log_plugin)
            .set(window_plugin),
    )
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins(EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .insert_state(initial_state)
    .add_plugins(scenes::MenuPlugin)
    .add_plugins(scenes::ScenePlugin::new("main"))
    .run();
}
