use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::scenes::{
    config::{
        default_circle_color_name, default_circle_radius, default_circle_rgb, default_color_name,
        default_color_rgb,
        ActiveScene, CameraConfig, CircleConfig, CubeConfig, InputConfig,
    },
    input::{apply_camera_input, resolve_camera_input_config, SceneCamera, SceneInputConfig},
    loaders::{
        load_camera_config, load_circle_config, load_cube_config, load_input_config, load_light_config,
        load_world_config,
    },
    world::{EntityPlacement, WorldConfig},
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
    let world_config: WorldConfig = load_world_config(&active_scene.name);
    let light_config = load_light_config(&active_scene.name);

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

    spawn_world_entities(
        &world_config,
        &circle_config,
        &cube_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    spawn_world_entities(
        &world_config,
        &circle_config,
        &cube_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    // lights
    spawn_lights(&light_config, &mut commands);

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

fn spawn_world_entities(
    world: &WorldConfig,
    circle_template: &CircleConfig,
    cube_template: &CubeConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    for entity in &world.entities {
        match entity.template.as_str() {
            path if path.ends_with("circle.toml") => spawn_circle(
                entity,
                circle_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            path if path.ends_with("cube.toml") => spawn_cube(
                entity,
                cube_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            other => {
                warn!(
                    "Unknown template '{other}' in world; skipping entity placement in scene '{}'.",
                    active_scene.name
                );
            }
        }
    }
}

fn spawn_circle(
    placement: &EntityPlacement,
    template: &CircleConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let circle_rgb =
        crate::scenes::config::parse_color(&template.color).unwrap_or_else(|| {
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
        placement.transform.rotation.roll.to_radians(),
        placement.transform.rotation.pitch.to_radians(),
        placement.transform.rotation.yaw.to_radians(),
    );
    commands.spawn((
        Name::new(
            placement
                .name_override
                .clone()
                .unwrap_or_else(|| template.name.clone()),
        ),
        Mesh3d(meshes.add(Circle::new(
            placement.radius.unwrap_or_else(default_circle_radius),
        ))),
        MeshMaterial3d(circle_material),
        Transform::from_xyz(
            placement.transform.position.x,
            placement.transform.position.y,
            placement.transform.position.z,
        )
        .with_rotation(circle_rotation)
        .with_scale(Vec3::splat(placement.transform.scale)),
    ));
}

fn spawn_lights(light_config: &crate::scenes::config::LightConfig, commands: &mut Commands) {
    let mut ambient_set = false;
    for light in &light_config.lights {
        let color = crate::scenes::config::parse_color(&light.color).unwrap_or([255, 255, 255]);
        let color = Color::srgb_u8(color[0], color[1], color[2]);
        match light.kind {
            crate::scenes::config::LightKind::Ambient => {
                if ambient_set {
                    warn!("Multiple ambient lights specified; only the first is applied.");
                    continue;
                }
                commands.insert_resource(AmbientLight {
                    color,
                    brightness: light.brightness,
                    affects_lightmapped_meshes: true,
                });
                ambient_set = true;
            }
            crate::scenes::config::LightKind::Point => {
                commands.spawn((
                    PointLight {
                        intensity: light.intensity,
                        range: light.range.unwrap_or(20.0),
                        shadows_enabled: light.shadows,
                        color,
                        ..default()
                    },
                    Transform::from_xyz(light.position.x, light.position.y, light.position.z),
                ));
            }
            crate::scenes::config::LightKind::Directional => {
                // Directional light uses rotation; look_at if provided
                let mut transform = Transform::default();
                if let Some(target) = &light.look_at {
                    transform = Transform::from_translation(Vec3::ZERO)
                        .looking_at(
                            Vec3::new(target.x, target.y, target.z),
                            Vec3::Y,
                        );
                }
                commands.spawn((
                    DirectionalLight {
                        illuminance: light.intensity,
                        shadows_enabled: light.shadows,
                        color,
                        ..default()
                    },
                    transform,
                ));
            }
        }
    }
    if !ambient_set {
        commands.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.0,
            affects_lightmapped_meshes: true,
        });
    }
}

fn spawn_cube(
    placement: &EntityPlacement,
    template: &CubeConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let cube_rgb = crate::scenes::config::parse_color(&template.color).unwrap_or_else(|| {
        warn!(
            "Falling back to default color '{}' for cube in scene '{}'.",
            default_color_name(),
            active_scene.name
        );
        default_color_rgb()
    });
    info!(
        "Applying cube rotation (deg) roll: {}, pitch: {}, yaw: {}",
        placement.transform.rotation.roll,
        placement.transform.rotation.pitch,
        placement.transform.rotation.yaw
    );
    let cube_rotation = Quat::from_euler(
        EulerRot::XYZ, // roll -> X, pitch -> Y, yaw -> Z
        placement.transform.rotation.roll.to_radians(),
        placement.transform.rotation.pitch.to_radians(),
        placement.transform.rotation.yaw.to_radians(),
    );
    let cube_material = materials.add(Color::srgb_u8(cube_rgb[0], cube_rgb[1], cube_rgb[2]));
    commands.spawn((
        Name::new(
            placement
                .name_override
                .clone()
                .unwrap_or_else(|| template.name.clone()),
        ),
        Mesh3d(meshes.add(Cuboid::new(
            template.dimensions.width,
            template.dimensions.height,
            template.dimensions.depth,
        ))),
        MeshMaterial3d(cube_material),
        Transform::from_xyz(
            placement.transform.position.x,
            placement.transform.position.y,
            placement.transform.position.z,
        )
        .with_rotation(cube_rotation)
        .with_scale(Vec3::splat(template.size.uniform_scale * placement.transform.scale)),
    ));
}
