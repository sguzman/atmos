use bevy::prelude::*;

pub const SCENE_ROOT: &str = "assets/scenes";
#[allow(dead_code)]
pub const OVERLAY_ROOT: &str = "assets/overlay";

#[derive(Resource, Debug, Clone)]
pub struct ActiveScene {
    pub name: String,
}
