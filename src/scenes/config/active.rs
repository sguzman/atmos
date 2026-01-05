use bevy::prelude::*;

pub const SCENE_ROOT: &str = "config/scenes";
#[allow(dead_code)]
pub const OVERLAY_ROOT: &str = "config/overlay";

#[derive(Resource, Debug, Clone)]
pub struct ActiveScene {
    pub name: String,
}
