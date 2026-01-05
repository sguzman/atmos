use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

pub(crate) fn configure_main_cursor(mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = windows.single_mut() {
        #[cfg(target_arch = "wasm32")]
        {
            cursor.grab_mode = CursorGrabMode::None;
            cursor.visible = true;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            cursor.grab_mode = CursorGrabMode::Locked;
            cursor.visible = false;
        }
        cursor.hit_test = true;
    }
}
