use bevy::{
    log::{info, warn},
    prelude::*,
};
use serde::Deserialize;
use std::fs;

const SCENE_ROOT: &str = "assets/scenes";

#[derive(Resource, Debug, Clone)]
pub struct ActiveScene {
    pub name: String,
}

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
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(
    active_scene: Res<ActiveScene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!(
        "Bootstrapping scene '{}' with inspector overlay enabled.",
        active_scene.name
    );

    let cube_config = load_cube_config(&active_scene.name);
    let camera_config = load_camera_config(&active_scene.name);

    if cube_config.physics.enabled {
        warn!(
            "Cube physics config is enabled for scene '{}' but not applied yet (body_type={}, mass={}, restitution={}, friction={}).",
            active_scene.name,
            cube_config.physics.body_type,
            cube_config.physics.mass,
            cube_config.physics.restitution,
            cube_config.physics.friction,
        );
    }

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cube from config
    let cube_rgb = parse_color(&cube_config.color).unwrap_or_else(|| {
        warn!(
            "Falling back to default color '{}' for cube in scene '{}'.",
            default_color_name(),
            active_scene.name
        );
        default_color_rgb()
    });
    let cube_material = materials.add(Color::srgb_u8(
        cube_rgb[0],
        cube_rgb[1],
        cube_rgb[2],
    ));
    commands.spawn((
        Name::new(cube_config.name),
        Mesh3d(meshes.add(Cuboid::new(
            cube_config.dimensions.width,
            cube_config.dimensions.height,
            cube_config.dimensions.depth,
        ))),
        MeshMaterial3d(cube_material),
        Transform::from_xyz(
            cube_config.position.x,
            cube_config.position.y,
            cube_config.position.z,
        )
        .with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            cube_config.rotation.yaw.to_radians(),
            cube_config.rotation.pitch.to_radians(),
            cube_config.rotation.roll.to_radians(),
        ))
        .with_scale(Vec3::splat(cube_config.size.uniform_scale)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Name::new(camera_config.name),
        Camera3d::default(),
        Transform::from_xyz(
            camera_config.transform.position.x,
            camera_config.transform.position.y,
            camera_config.transform.position.z,
        )
        .looking_at(
            Vec3::new(
                camera_config.transform.look_at.x,
                camera_config.transform.look_at.y,
                camera_config.transform.look_at.z,
            ),
            Vec3::new(
                camera_config.transform.up.x,
                camera_config.transform.up.y,
                camera_config.transform.up.z,
            ),
        ),
    ));
}

fn load_cube_config(scene: &str) -> CubeConfig {
    let path = cube_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return CubeConfig::default();
        }
    };

    match toml::from_str::<CubeConfig>(&contents) {
        Ok(config) => {
            info!("Loaded cube config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            CubeConfig::default()
        }
    }
}

fn load_camera_config(scene: &str) -> CameraConfig {
    let path = camera_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return CameraConfig::default();
        }
    };

    match toml::from_str::<CameraConfig>(&contents) {
        Ok(config) => {
            info!("Loaded camera config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            CameraConfig::default()
        }
    }
}

fn cube_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/cube.toml")
}

fn camera_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/camera.toml")
}

#[derive(Debug, Deserialize)]
struct CubeConfig {
    name: String,
    #[serde(default = "default_color_name")]
    color: String,
    #[serde(default)]
    position: PositionConfig,
    #[serde(default)]
    rotation: RotationConfig,
    #[serde(default)]
    dimensions: DimensionsConfig,
    #[serde(default)]
    size: SizeConfig,
    #[serde(default)]
    physics: PhysicsConfig,
}

impl Default for CubeConfig {
    fn default() -> Self {
        Self {
            name: "cube".to_string(),
            color: default_color_name(),
            position: PositionConfig::default(),
            rotation: RotationConfig::default(),
            dimensions: DimensionsConfig::default(),
            size: SizeConfig::default(),
            physics: PhysicsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CameraConfig {
    name: String,
    #[serde(default)]
    transform: TransformConfig,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            name: "main_camera".to_string(),
            transform: TransformConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TransformConfig {
    #[serde(default = "default_camera_position")]
    position: Vec3Config,
    #[serde(default = "default_camera_look_at")]
    look_at: Vec3Config,
    #[serde(default = "default_camera_up")]
    up: Vec3Config,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            position: default_camera_position(),
            look_at: default_camera_look_at(),
            up: default_camera_up(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Vec3Config {
    #[serde(default)]
    x: f32,
    #[serde(default)]
    y: f32,
    #[serde(default)]
    z: f32,
}

impl Default for Vec3Config {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[derive(Debug, Deserialize)]
struct RotationConfig {
    #[serde(default)]
    roll: f32,
    #[serde(default)]
    pitch: f32,
    #[serde(default)]
    yaw: f32,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct PositionConfig {
    #[serde(default)]
    x: f32,
    #[serde(default)]
    y: f32,
    #[serde(default)]
    z: f32,
}

#[derive(Debug, Deserialize)]
struct DimensionsConfig {
    #[serde(default = "default_unit")]
    width: f32,
    #[serde(default = "default_unit")]
    height: f32,
    #[serde(default = "default_unit")]
    depth: f32,
}

impl Default for DimensionsConfig {
    fn default() -> Self {
        let unit = default_unit();
        Self {
            width: unit,
            height: unit,
            depth: unit,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SizeConfig {
    #[serde(default = "default_unit")]
    uniform_scale: f32,
}

impl Default for SizeConfig {
    fn default() -> Self {
        Self {
            uniform_scale: default_unit(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct PhysicsConfig {
    #[serde(default)]
    enabled: bool,
    #[serde(default = "default_body_type")]
    body_type: String,
    #[serde(default = "default_mass")]
    mass: f32,
    #[serde(default)]
    restitution: f32,
    #[serde(default = "default_friction")]
    friction: f32,
}

fn default_unit() -> f32 {
    1.0
}

fn default_body_type() -> String {
    "dynamic".to_string()
}

fn default_mass() -> f32 {
    1.0
}

fn default_friction() -> f32 {
    0.5
}

fn default_camera_position() -> Vec3Config {
    Vec3Config {
        x: -2.5,
        y: 4.5,
        z: 9.0,
    }
}

fn default_camera_look_at() -> Vec3Config {
    Vec3Config { x: 0.0, y: 0.0, z: 0.0 }
}

fn default_camera_up() -> Vec3Config {
    Vec3Config { x: 0.0, y: 1.0, z: 0.0 }
}

fn parse_color(color_name: &str) -> Option<[u8; 3]> {
    match csscolorparser::parse(color_name) {
        Ok(parsed) => {
            let [r, g, b, _a] = parsed.to_rgba8();
            Some([r, g, b])
        }
        Err(err) => {
            warn!("Failed to parse color '{color_name}': {err}");
            None
        }
    }
}

fn default_color_name() -> String {
    "red".to_string()
}

fn default_color_rgb() -> [u8; 3] {
    parse_color(&default_color_name()).unwrap_or([255, 0, 0])
}
