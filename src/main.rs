mod app_config;
mod scenes;

use app_config::load_app_config;
#[cfg(target_arch = "wasm32")]
use app_config::load_wasm_config;
use bevy::asset::{AssetApp, AssetMetaCheck, AssetPlugin};
#[cfg(target_arch = "wasm32")]
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use bevy_rapier3d::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
#[cfg(not(target_arch = "wasm32"))]
use clap::{Parser, Subcommand};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let cli = Cli::parse();
    let app_config = load_app_config();
    run_app(app_config, cli.allow_runtime_mesh, cli.command);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let mut app_config = load_app_config();
    let wasm_config = load_wasm_config();
    app_config.apply_wasm_config(&wasm_config);
    run_app(app_config, wasm_config.allow_runtime_mesh, None);
}

fn run_app(
    app_config: app_config::AppConfig,
    allow_runtime_mesh: bool,
    command: Option<Commands>,
) {
    let log_plugin = app_config.to_log_plugin();
    let window_plugin = app_config.to_window_plugin();

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(Commands::Bake { scene }) = command {
            let settings = scenes::MeshCacheSettings::default();
            if let Err(err) = bake_mesh_cache(scene.as_deref(), &settings) {
                eprintln!("Bake failed: {err}");
                std::process::exit(1);
            }
            return;
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = command;

    let mut app = App::new();

    app.insert_resource::<WinitSettings>(app_config.winit_settings());
    app.insert_resource(app_config.clone());
    app.insert_resource(scenes::MeshCacheSettings::new(
        allow_runtime_mesh && matches!(app_config.mode, app_config::AppMode::Dev),
    ));
    app.init_resource::<scenes::TomlCache>();

    let default_plugins = DefaultPlugins
        .set(log_plugin)
        .set(window_plugin)
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        });
    #[cfg(target_arch = "wasm32")]
    let default_plugins = default_plugins.disable::<AudioPlugin>();
    app.add_plugins(default_plugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    if app_config.debug.rapier_debug {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    if app_config.debug.inspector {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.init_asset::<scenes::TomlAsset>()
        .init_asset_loader::<scenes::TomlAssetLoader>()
        .init_asset_loader::<scenes::MeshCacheLoader>()
        .insert_state(scenes::AppState::default())
        .add_plugins(scenes::MenuPlugin)
        .add_plugins(scenes::ScenePlugin::new("main"))
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long)]
    allow_runtime_mesh: bool,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Subcommand)]
enum Commands {
    Bake {
        #[arg(long)]
        scene: Option<String>,
    },
}

#[cfg(target_arch = "wasm32")]
enum Commands {}

fn bake_mesh_cache(scene: Option<&str>, settings: &scenes::MeshCacheSettings) -> Result<(), String> {
    scenes::bake_meshes(scene, settings).map_err(|err| err.to_string())
}
