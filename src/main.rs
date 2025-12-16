mod app_config;
mod scenes;

use app_config::load_app_config;
use bevy::prelude::*;
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
    .add_plugins(EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(scenes::ScenePlugin::new("main"))
    .run();
}
