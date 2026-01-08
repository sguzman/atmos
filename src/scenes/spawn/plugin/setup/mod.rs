use bevy::prelude::*;
use bevy_rapier3d::prelude::{DefaultRapierContext, RapierConfiguration, RapierContextSimulation};

use crate::app_config::AppConfig;
use crate::scenes::{
    config::ActiveScene,
    loaders::TomlCache,
    MeshCacheSettings, TomlAsset,
};

mod actions;
mod input;
mod player;
mod world;

#[derive(Resource, Default)]
pub(crate) struct SceneSetupState {
    pub done: bool,
}

pub(crate) fn reset_scene_setup_state(mut commands: Commands) {
    commands.insert_resource(SceneSetupState::default());
}

pub(crate) fn setup_scene(
    active_scene: Res<ActiveScene>,
    app_config: Res<AppConfig>,
    mesh_cache: Res<MeshCacheSettings>,
    mut setup_state: ResMut<SceneSetupState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    toml_assets: Res<Assets<TomlAsset>>,
    mut toml_cache: ResMut<TomlCache>,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    mut rapier_context: Query<&mut RapierContextSimulation, With<DefaultRapierContext>>,
) {
    if setup_state.done {
        return;
    }

    let input_config = match input::load_scene_input(
        &active_scene,
        &mut commands,
        &mut toml_cache,
        &asset_server,
        &toml_assets,
    ) {
        Some(config) => config,
        None => return,
    };

    let (world_config, entities_config) = match world::load_world_and_entities(
        &active_scene,
        &mut commands,
        &mut toml_cache,
        &asset_server,
        &toml_assets,
    ) {
        Some(configs) => configs,
        None => return,
    };

    let initial_noclip = match actions::setup_actions(
        &input_config,
        &active_scene,
        &mesh_cache,
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        &toml_assets,
        &mut toml_cache,
    ) {
        Some(value) => value,
        None => return,
    };

    if !world::spawn_world_content(
        &world_config,
        &entities_config,
        &active_scene,
        &mesh_cache,
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        &toml_assets,
        &mut toml_cache,
        &mut rapier_config,
        &mut rapier_context,
    ) {
        return;
    }

    player::spawn_player_and_cameras(
        &world_config,
        &app_config,
        initial_noclip,
        &mut commands,
    );

    info!("Bootstrapping scene '{}' complete.", active_scene.name);
    setup_state.done = true;
}
