mod app_config;
mod scenes;
mod simulation;

use app_config::load_app_config;
use bevy::asset::{AssetApp, AssetMetaCheck, AssetPlugin};
use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();
    let app_config = load_app_config();
    run_app(
        app_config,
        cli.allow_runtime_mesh,
        cli.agent_demo,
        cli.command,
    );
}

fn run_app(
    app_config: app_config::AppConfig,
    allow_runtime_mesh: bool,
    agent_demo: bool,
    command: Option<Commands>,
) {
    let log_plugin = app_config.to_log_plugin();
    let window_plugin = app_config.to_window_plugin();

    if let Some(Commands::Bake { scene }) = command {
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

    let default_plugins = DefaultPlugins
        .set(log_plugin)
        .set(window_plugin)
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        });
    app.add_plugins(default_plugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    if app_config.debug.rapier_debug {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    if app_config.debug.inspector {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }

    if agent_demo {
        app.add_plugins(simulation::AgentSimulationPlugin::default())
            .run();
        return;
    }

    app.insert_resource(scenes::MeshCacheSettings::new(
        allow_runtime_mesh && matches!(app_config.mode, app_config::AppMode::Dev),
    ));
    app.init_resource::<scenes::TomlCache>();

    let initial_state = initial_state_from_world();

    app.init_asset::<scenes::TomlAsset>()
        .init_asset_loader::<scenes::TomlAssetLoader>()
        .init_asset_loader::<scenes::MeshCacheLoader>()
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
    /// Run the small autonomous-agent simulation testbed instead of the normal scene.
    #[arg(long)]
    agent_demo: bool,
}

#[derive(Subcommand)]
enum Commands {
    Bake {
        #[arg(long)]
        scene: Option<String>,
    },
}

fn bake_mesh_cache(
    scene: Option<&str>,
    settings: &scenes::MeshCacheSettings,
) -> Result<(), String> {
    scenes::bake_meshes(scene, settings).map_err(|err| err.to_string())
}

fn initial_state_from_world() -> scenes::AppState {
    let contents = load_world_startup_toml();
    if let Some(contents) = contents {
        if let Ok(cfg) = toml::from_str::<scenes::WorldConfig>(&contents) {
            if let Some(scene) = cfg.startup_scene.as_deref() {
                return scenes::AppState::from_scene_name(scene);
            }
        }
    }
    scenes::AppState::default()
}

fn load_world_startup_toml() -> Option<String> {
    std::fs::read_to_string("assets/scenes/main/world.toml").ok()
}
