use super::SCENE_ROOT;

pub fn cube_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/cube.3D.toml")
}

pub fn circle_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/circle.2D.toml")
}

pub fn rectangle_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/rectangle.3D.toml")
}

pub fn sphere_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/sphere.3D.toml")
}

pub fn top_light_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/top_light.light.toml")
}

pub fn pillar_combo_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/combo/pillar_with_light.toml")
}

pub fn camera_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/camera.toml")
}

pub fn input_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/input.toml")
}

pub fn bounding_box_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/boundingbox.toml")
}

pub fn action_config_path(scene: &str, action_path: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/{action_path}")
}

pub fn light_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/light.toml")
}

pub fn sun_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/sun.toml")
}

pub fn skybox_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/skybox.toml")
}

#[allow(dead_code)]
pub fn overlay_config_path(name: &str) -> String {
    format!(
        "{overlay_root}/{name}.toml",
        overlay_root = crate::scenes::config::OVERLAY_ROOT
    )
}
