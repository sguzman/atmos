use bevy::{
    input::ButtonInput,
    input::keyboard::KeyCode,
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

#[derive(Resource, Debug, Clone)]
struct SceneInputConfig {
    camera: ResolvedCameraInputConfig,
}

#[derive(Debug, Clone)]
struct ResolvedCameraInputConfig {
    movement: ResolvedMovementConfig,
    rotation: ResolvedRotationConfig,
}

#[derive(Debug, Clone)]
struct ResolvedMovementConfig {
    speed: f32,
    forward: Option<KeyCode>,
    backward: Option<KeyCode>,
    left: Option<KeyCode>,
    right: Option<KeyCode>,
}

#[derive(Debug, Clone)]
struct ResolvedRotationConfig {
    degrees_per_second: f32,
    yaw_left: Option<KeyCode>,
    yaw_right: Option<KeyCode>,
    pitch_up: Option<KeyCode>,
    pitch_down: Option<KeyCode>,
}

#[derive(Component)]
struct SceneCamera;

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
        app.add_systems(Update, apply_camera_input);
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

    let input_config = load_input_config(&active_scene.name);
    let resolved_input = resolve_input_config(&input_config);
    commands.insert_resource(SceneInputConfig {
        camera: resolved_input,
    });

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
    info!(
        "Applying cube rotation (deg) roll: {}, pitch: {}, yaw: {}",
        cube_config.rotation.roll, cube_config.rotation.pitch, cube_config.rotation.yaw
    );
    let cube_rotation = Quat::from_euler(
        EulerRot::XYZ, // roll -> X, pitch -> Y, yaw -> Z
        cube_config.rotation.roll.to_radians(),
        cube_config.rotation.pitch.to_radians(),
        cube_config.rotation.yaw.to_radians(),
    );
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
        .with_rotation(cube_rotation)
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
        SceneCamera,
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

fn apply_camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneInputConfig>>,
    mut cameras: Query<&mut Transform, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

    for mut transform in cameras.iter_mut() {
        let move_cfg = &config.camera.movement;
        let rot_cfg = &config.camera.rotation;
        let dt = time.delta_secs();

        // Movement
        let mut forward_axis = 0.0;
        let mut right_axis = 0.0;
        if let Some(key) = move_cfg.forward {
            if keys.pressed(key) {
                forward_axis += 1.0;
            }
        }
        if let Some(key) = move_cfg.backward {
            if keys.pressed(key) {
                forward_axis -= 1.0;
            }
        }
        if let Some(key) = move_cfg.right {
            if keys.pressed(key) {
                right_axis += 1.0;
            }
        }
        if let Some(key) = move_cfg.left {
            if keys.pressed(key) {
                right_axis -= 1.0;
            }
        }

        if forward_axis != 0.0 || right_axis != 0.0 {
            let forward = transform.rotation * -Vec3::Z;
            let right = transform.rotation * Vec3::X;
            let mut direction = forward * forward_axis + right * right_axis;
            if direction.length_squared() > 0.0 {
                direction = direction.normalize();
                transform.translation += direction * move_cfg.speed * dt;
            }
        }

        // Rotation
        let yaw_amount = {
            let mut val = 0.0;
            if let Some(key) = rot_cfg.yaw_left {
                if keys.pressed(key) {
                    val += 1.0;
                }
            }
            if let Some(key) = rot_cfg.yaw_right {
                if keys.pressed(key) {
                    val -= 1.0;
                }
            }
            val
        };

        let pitch_amount = {
            let mut val = 0.0;
            if let Some(key) = rot_cfg.pitch_up {
                if keys.pressed(key) {
                    val += 1.0;
                }
            }
            if let Some(key) = rot_cfg.pitch_down {
                if keys.pressed(key) {
                    val -= 1.0;
                }
            }
            val
        };

        let rot_speed = rot_cfg.degrees_per_second.to_radians() * dt;
        if yaw_amount != 0.0 {
            transform.rotate_y(yaw_amount * rot_speed);
        }
        if pitch_amount != 0.0 {
            transform.rotate_local_x(pitch_amount * rot_speed);
        }
    }
}

fn resolve_input_config(config: &InputConfig) -> ResolvedCameraInputConfig {
    let movement = &config.camera.movement;
    let rotation = &config.camera.rotation;

    ResolvedCameraInputConfig {
        movement: ResolvedMovementConfig {
            speed: movement.speed,
            forward: resolve_key_or_warn(&movement.forward, "camera forward"),
            backward: resolve_key_or_warn(&movement.backward, "camera backward"),
            left: resolve_key_or_warn(&movement.left, "camera left"),
            right: resolve_key_or_warn(&movement.right, "camera right"),
        },
        rotation: ResolvedRotationConfig {
            degrees_per_second: rotation.degrees_per_second,
            yaw_left: resolve_key_or_warn(&rotation.yaw_left, "camera yaw left"),
            yaw_right: resolve_key_or_warn(&rotation.yaw_right, "camera yaw right"),
            pitch_up: resolve_key_or_warn(&rotation.pitch_up, "camera pitch up"),
            pitch_down: resolve_key_or_warn(&rotation.pitch_down, "camera pitch down"),
        },
    }
}

fn resolve_key_or_warn(key: &str, action: &str) -> Option<KeyCode> {
    if key.trim().is_empty() {
        return None;
    }
    match resolve_key(key) {
        Some(code) => Some(code),
        None => {
            warn!("Unrecognized key '{key}' for {action}; binding disabled.");
            None
        }
    }
}

fn resolve_key(key: &str) -> Option<KeyCode> {
    let normalized = key.trim().to_ascii_uppercase();
    match normalized.as_str() {
        "W" => Some(KeyCode::KeyW),
        "A" => Some(KeyCode::KeyA),
        "S" => Some(KeyCode::KeyS),
        "D" => Some(KeyCode::KeyD),
        "ARROWLEFT" | "LEFT" => Some(KeyCode::ArrowLeft),
        "ARROWRIGHT" | "RIGHT" => Some(KeyCode::ArrowRight),
        "ARROWUP" | "UP" => Some(KeyCode::ArrowUp),
        "ARROWDOWN" | "DOWN" => Some(KeyCode::ArrowDown),
        _ => None,
    }
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

fn load_input_config(scene: &str) -> InputConfig {
    let path = input_config_path(scene);
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {path}: {err}. Falling back to defaults.");
            return InputConfig::default();
        }
    };

    match toml::from_str::<InputConfig>(&contents) {
        Ok(config) => {
            info!("Loaded input config from {path}.");
            config
        }
        Err(err) => {
            warn!("Failed to parse {path}: {err}. Falling back to defaults.");
            InputConfig::default()
        }
    }
}

fn cube_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/entities/cube.toml")
}

fn camera_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/camera.toml")
}

fn input_config_path(scene: &str) -> String {
    format!("{SCENE_ROOT}/{scene}/input.toml")
}

#[derive(Debug, Deserialize)]
struct InputConfig {
    #[serde(default)]
    camera: CameraInputConfig,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            camera: CameraInputConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CameraInputConfig {
    #[serde(default)]
    movement: MovementConfig,
    #[serde(default)]
    rotation: CameraRotationConfig,
}

impl Default for CameraInputConfig {
    fn default() -> Self {
        Self {
            movement: MovementConfig::default(),
            rotation: CameraRotationConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct MovementConfig {
    #[serde(default = "default_move_speed")]
    speed: f32,
    #[serde(default = "default_forward_key")]
    forward: String,
    #[serde(default = "default_backward_key")]
    backward: String,
    #[serde(default = "default_left_key")]
    left: String,
    #[serde(default = "default_right_key")]
    right: String,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            speed: default_move_speed(),
            forward: default_forward_key(),
            backward: default_backward_key(),
            left: default_left_key(),
            right: default_right_key(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CameraRotationConfig {
    #[serde(default = "default_rotation_speed")]
    degrees_per_second: f32,
    #[serde(default = "default_yaw_left_key")]
    yaw_left: String,
    #[serde(default = "default_yaw_right_key")]
    yaw_right: String,
    #[serde(default = "default_pitch_up_key")]
    pitch_up: String,
    #[serde(default = "default_pitch_down_key")]
    pitch_down: String,
}

impl Default for CameraRotationConfig {
    fn default() -> Self {
        Self {
            degrees_per_second: default_rotation_speed(),
            yaw_left: default_yaw_left_key(),
            yaw_right: default_yaw_right_key(),
            pitch_up: default_pitch_up_key(),
            pitch_down: default_pitch_down_key(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CubeConfig {
    name: String,
    #[serde(default = "default_color_name")]
    color: String,
    #[serde(default)]
    position: PositionConfig,
    #[serde(default)]
    rotation: CubeRotationConfig,
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
            rotation: CubeRotationConfig::default(),
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
struct CubeRotationConfig {
    #[serde(default)]
    roll: f32,
    #[serde(default)]
    pitch: f32,
    #[serde(default)]
    yaw: f32,
}

impl Default for CubeRotationConfig {
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

fn default_move_speed() -> f32 {
    6.0
}

fn default_rotation_speed() -> f32 {
    90.0
}

fn default_forward_key() -> String {
    "W".to_string()
}

fn default_backward_key() -> String {
    "S".to_string()
}

fn default_left_key() -> String {
    "A".to_string()
}

fn default_right_key() -> String {
    "D".to_string()
}

fn default_yaw_left_key() -> String {
    "ArrowLeft".to_string()
}

fn default_yaw_right_key() -> String {
    "ArrowRight".to_string()
}

fn default_pitch_up_key() -> String {
    "ArrowUp".to_string()
}

fn default_pitch_down_key() -> String {
    "ArrowDown".to_string()
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
