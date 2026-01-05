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

use super::logging::{log_after_setup, reset_scene_log_state};
use super::overlay::{reset_overlay_spawn_state, spawn_overlays_from_config};

mod cursor;
mod cleanup;
mod overlays;
mod render;
mod setup;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub(crate) use setup::SceneSetupState;

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
        app.init_resource::<setup::SceneSetupState>();
        app.init_resource::<super::logging::SceneLogState>();
        app.add_systems(OnEnter(AppState::Main), setup::reset_scene_setup_state);
        app.add_systems(OnEnter(AppState::Main), reset_scene_log_state);
        app.add_systems(OnEnter(AppState::Main), reset_overlay_spawn_state);
        app.add_systems(OnEnter(AppState::Main), cursor::configure_main_cursor);
        app.add_systems(OnExit(AppState::Main), cleanup::cleanup_main_scene);
        app.add_systems(Update, setup::setup_scene.run_if(in_state(AppState::Main)));
        app.add_systems(Update, log_after_setup.run_if(in_state(AppState::Main)));
        app.add_systems(Update, spawn_overlays_from_config.run_if(in_state(AppState::Main)));
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

        #[cfg(target_arch = "wasm32")]
        {
            app.add_systems(OnEnter(AppState::Main), wasm::reset_wasm_cursor_state);
            app.add_systems(OnEnter(AppState::Main), wasm::spawn_wasm_kill_button);
            app.add_systems(OnExit(AppState::Main), wasm::cleanup_wasm_kill_button);
            app.add_systems(Update, wasm::handle_wasm_kill_button.run_if(in_state(AppState::Main)));
            app.add_systems(Update, wasm::lock_cursor_on_click.run_if(in_state(AppState::Main)));
        }
    }
}
