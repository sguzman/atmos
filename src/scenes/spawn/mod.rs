mod combo;
mod cut;
mod entities;
mod lights;
mod logging;
mod overlay;
mod plugin;
mod sun;
mod world;

use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct SceneEntityTag;

pub use cut::CuttableShape;
pub use overlay::{OverlayTag, reset_overlay_spawn_state, spawn_overlays_from_config};
pub use plugin::ScenePlugin;
pub use sun::SunLight;
