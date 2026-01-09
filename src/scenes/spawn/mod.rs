mod entities;
mod combo;
mod lights;
mod logging;
mod plugin;
mod sun;
mod world;
mod overlay;

use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct SceneEntityTag;

pub use plugin::ScenePlugin;
pub use overlay::{reset_overlay_spawn_state, spawn_overlays_from_config, OverlayTag};
pub use sun::SunLight;
