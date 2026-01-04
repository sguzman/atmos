use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Main,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Main
    }
}

impl AppState {
    pub fn from_scene_name(name: &str) -> Self {
        if name.trim().eq_ignore_ascii_case("menu") {
            Self::Menu
        } else {
            Self::Main
        }
    }
}
