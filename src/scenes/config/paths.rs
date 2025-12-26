use super::SCENE_ROOT;

pub fn input_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/input.toml")
}

pub fn action_config_path(scene: &str, action_path: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/{action_path}")
}

pub fn overlay_config_path(name: &str) -> String {
    format!(
        "{overlay_root}/{name}.toml",
        overlay_root = crate::scenes::config::OVERLAY_ROOT
    )
}
