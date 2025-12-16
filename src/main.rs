mod app_config;
mod scenes;

use app_config::load_app_config;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    let app_config = load_app_config();
    let log_plugin = app_config.to_log_plugin();
    let window_plugin = app_config.to_window_plugin();

    let mut app = App::new();

    if let Some(strategy) = app_config.time_update_strategy() {
        app.insert_resource(strategy);
    }
    app.insert_resource(app_config);

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
