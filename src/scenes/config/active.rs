use bevy::prelude::*;

pub const SCENE_ROOT: &str = "assets/scenes";

#[derive(Resource, Debug, Clone)]
pub struct ActiveScene {
    pub name: String,
}
