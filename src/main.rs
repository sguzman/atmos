use bevy::{
    log::{info, warn},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use serde::Deserialize;
use std::fs;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Bootstrapping 3D scene with inspector overlay enabled.");
    let cube_config = load_cube_config();
    let camera_config = load_camera_config();
    if cube_config.physics.enabled {
        warn!(
            "Cube physics config is enabled but not applied yet (body_type={}, mass={}, restitution={}, friction={}).",
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
    let cube_material = materials.add(Color::srgb_u8(
        cube_config.color.srgb[0],
        cube_config.color.srgb[1],
        cube_config.color.srgb[2],
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

fn load_cube_config() -> CubeConfig {
    const CONFIG_PATH: &str = "assets/entities/cube.toml";
    let contents = match fs::read_to_string(CONFIG_PATH) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {CONFIG_PATH}: {err}. Falling back to defaults.");
            return CubeConfig::default();
        }
    };

    match toml::from_str::<CubeConfig>(&contents) {
        Ok(config) => {
            info!("Loaded cube config from {CONFIG_PATH}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {CONFIG_PATH}: {err}. Falling back to defaults.");
            CubeConfig::default()
        }
    }
}

fn load_camera_config() -> CameraConfig {
    const CONFIG_PATH: &str = "assets/camera.toml";
    let contents = match fs::read_to_string(CONFIG_PATH) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {CONFIG_PATH}: {err}. Falling back to defaults.");
            return CameraConfig::default();
        }
    };

    match toml::from_str::<CameraConfig>(&contents) {
        Ok(config) => {
            info!("Loaded camera config from {CONFIG_PATH}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {CONFIG_PATH}: {err}. Falling back to defaults.");
            CameraConfig::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct CubeConfig {
    name: String,
    #[serde(default)]
    color: ColorConfig,
    #[serde(default)]
    position: PositionConfig,
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
            color: ColorConfig::default(),
            position: PositionConfig::default(),
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
struct ColorConfig {
    #[serde(default = "default_srgb")]
    srgb: [u8; 3],
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self { srgb: default_srgb() }
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

fn default_srgb() -> [u8; 3] {
    [124, 144, 255]
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
