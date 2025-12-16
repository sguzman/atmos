use super::SCENE_ROOT;

pub fn cube_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/cube.toml")
}

pub fn circle_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/circle.toml")
}

pub fn rectangle_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/rectangle.toml")
}

pub fn top_light_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/top_light.toml")
}

pub fn pillar_combo_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/pillar_with_light.toml")
}

pub fn camera_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/camera.toml")
}

pub fn input_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/input.toml")
}

pub fn light_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/light.toml")
}
