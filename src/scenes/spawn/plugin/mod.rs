use bevy::prelude::*;
use bevy::state::condition::in_state;

use crate::scenes::{
    bounds::despawn_out_of_bounds,
    config::ActiveScene,
    input::{
        apply_camera_input, apply_fov_action, apply_grab_action, apply_jump_action,
        apply_noclip_toggle, apply_player_respawn, apply_shoot_action, apply_sprint_toggle,
        apply_zoom_action, update_grab_hold, update_grab_hover,
    },
    AppState,
};

use super::logging::{log_camera, log_lights};
use super::overlay::spawn_overlays_from_config;

mod cursor;
mod overlays;
mod render;
mod setup;

pub struct ScenePlugin {
    scene: &'static str,
}

impl ScenePlugin {
    pub const fn new(scene: &'static str) -> Self {
        Self { scene }
    }
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveScene {
            name: self.scene.to_string(),
        });
        app.add_systems(OnEnter(AppState::Main), setup::setup_scene);
        app.add_systems(OnEnter(AppState::Main), cursor::configure_main_cursor);
        app.add_systems(
            OnEnter(AppState::Main),
            (log_lights, log_camera, spawn_overlays_from_config).after(setup::setup_scene),
        );
        app.add_systems(Update, apply_camera_input.run_if(in_state(AppState::Main)));
        app.add_systems(Update, apply_fov_action.run_if(in_state(AppState::Main)));
        app.add_systems(Update, update_grab_hover.run_if(in_state(AppState::Main)));
        app.add_systems(
            Update,
            apply_grab_action
                .after(update_grab_hover)
                .run_if(in_state(AppState::Main)),
        );
        app.add_systems(
            Update,
            update_grab_hold
                .after(apply_grab_action)
                .run_if(in_state(AppState::Main)),
        );
        app.add_systems(Update, apply_jump_action.run_if(in_state(AppState::Main)));
        app.add_systems(Update, apply_noclip_toggle.run_if(in_state(AppState::Main)));
        app.add_systems(Update, apply_player_respawn.run_if(in_state(AppState::Main)));
        app.add_systems(Update, apply_shoot_action.run_if(in_state(AppState::Main)));
        app.add_systems(Update, apply_sprint_toggle.run_if(in_state(AppState::Main)));
        app.add_systems(Update, apply_zoom_action.run_if(in_state(AppState::Main)));
        app.add_systems(Update, despawn_out_of_bounds.run_if(in_state(AppState::Main)));
        app.add_systems(Update, overlays::toggle_overlays.run_if(in_state(AppState::Main)));
    }
}
