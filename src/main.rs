mod app_config;
mod scenes;

use app_config::load_app_config;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
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

    app.add_plugins(
        DefaultPlugins
            .set(log_plugin)
            .set(window_plugin),
    )
    .add_systems(Startup, configure_cursor_options)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(scenes::ScenePlugin::new("main"))
    .run();
}

fn configure_cursor_options(mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = windows.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        cursor.hit_test = true;
    }
}
