use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::scenes::{
    config::{
        default_circle_color_name, default_circle_rgb, default_color_name, default_color_rgb,
        ActiveScene, CameraConfig, CircleConfig, CubeConfig, InputConfig,
    },
    input::{apply_camera_input, resolve_camera_input_config, SceneCamera, SceneInputConfig},
    loaders::{load_camera_config, load_circle_config, load_cube_config, load_input_config},
};

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

    let input_config: InputConfig = load_input_config(&active_scene.name);
    let camera_input =
        resolve_camera_input_config(&input_config.camera.movement, &input_config.camera.rotation);
    commands.insert_resource(SceneInputConfig {
        camera: camera_input,
    });

    let cube_config: CubeConfig = load_cube_config(&active_scene.name);
    let camera_config: CameraConfig = load_camera_config(&active_scene.name);
    let circle_config: CircleConfig = load_circle_config(&active_scene.name);

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

    // circular base from config
    let circle_rgb = crate::scenes::config::parse_color(&circle_config.color).unwrap_or_else(|| {
        warn!(
            "Falling back to default color '{}' for circle in scene '{}'.",
            default_circle_color_name(),
            active_scene.name
        );
        default_circle_rgb()
    });
    let circle_material =
        materials.add(Color::srgb_u8(circle_rgb[0], circle_rgb[1], circle_rgb[2]));
    let circle_rotation = Quat::from_euler(
        EulerRot::XYZ,
        circle_config.rotation.roll.to_radians(),
        circle_config.rotation.pitch.to_radians(),
        circle_config.rotation.yaw.to_radians(),
    );
    commands.spawn((
        Name::new(circle_config.name),
        Mesh3d(meshes.add(Circle::new(circle_config.radius))),
        MeshMaterial3d(circle_material),
        Transform::from_xyz(
            circle_config.position.x,
            circle_config.position.y,
            circle_config.position.z,
        )
        .with_rotation(circle_rotation),
    ));

    // cube from config
    let cube_rgb = crate::scenes::config::parse_color(&cube_config.color).unwrap_or_else(|| {
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
    let cube_material = materials.add(Color::srgb_u8(cube_rgb[0], cube_rgb[1], cube_rgb[2]));
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
