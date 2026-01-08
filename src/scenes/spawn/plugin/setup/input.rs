use bevy::prelude::*;

use crate::scenes::{
    config::{ActiveScene, InputConfig},
    input::{resolve_camera_input_config, resolve_overlay_toggles, SceneInputConfig},
    loaders::{load_input_config, ConfigLoad, TomlCache},
    TomlAsset,
};

pub(crate) fn load_scene_input(
    active_scene: &ActiveScene,
    commands: &mut Commands,
    toml_cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> Option<InputConfig> {
    let input_config: InputConfig = match load_input_config(
        &active_scene.name,
        toml_cache,
        asset_server,
        toml_assets,
    ) {
        ConfigLoad::Pending => return None,
        ConfigLoad::Ready(config) => config,
    };
    let camera_input = resolve_camera_input_config(
        &input_config.camera.movement,
        &input_config.camera.rotation,
    );
    commands.insert_resource(SceneInputConfig {
        camera: camera_input,
        overlays: resolve_overlay_toggles(&input_config.overlays),
    });
    Some(input_config)
}
