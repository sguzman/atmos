mod app_config;
mod scenes;

use app_config::load_app_config;
use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use bevy_rapier3d::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();
    let app_config = load_app_config();
    let log_plugin = app_config.to_log_plugin();
    let window_plugin = app_config.to_window_plugin();

    if let Some(Commands::Bake { scene }) = cli.command {
        let settings = scenes::MeshCacheSettings::default();
        if let Err(err) = bake_mesh_cache(scene.as_deref(), &settings) {
            eprintln!("Bake failed: {err}");
            std::process::exit(1);
        }
        return;
    }

    let mut app = App::new();

    app.insert_resource::<WinitSettings>(app_config.winit_settings());
    app.insert_resource(app_config.clone());
    app.insert_resource(scenes::MeshCacheSettings::new(
        cli.allow_runtime_mesh && matches!(app_config.mode, app_config::AppMode::Dev),
    ));

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
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    if app_config.debug.rapier_debug {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    if app_config.debug.inspector {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.init_asset_loader::<scenes::MeshCacheLoader>()
        .insert_state(initial_state)
        .add_plugins(scenes::MenuPlugin)
        .add_plugins(scenes::ScenePlugin::new("main"))
        .run();
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long)]
    allow_runtime_mesh: bool,
}

#[derive(Subcommand)]
enum Commands {
    Bake {
        #[arg(long)]
        scene: Option<String>,
    },
}

fn bake_mesh_cache(scene: Option<&str>, settings: &scenes::MeshCacheSettings) -> Result<(), String> {
    scenes::bake_meshes(scene, settings).map_err(|err| err.to_string())
}
