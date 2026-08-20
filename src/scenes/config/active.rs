use bevy::prelude::*;

// AssetServer paths are relative to the assets directory.
pub const SCENE_ROOT: &str = "scenes";
// Filesystem paths for native baking/config discovery.
pub const SCENE_FS_ROOT: &str = "assets/scenes";
#[allow(dead_code)]
pub const OVERLAY_ROOT: &str = "overlay";
#[allow(dead_code)]
pub const DIALOGUE_ROOT: &str = "dialogue";

#[derive(Resource, Debug, Clone)]
pub struct ActiveScene {
    pub name: String,
}
