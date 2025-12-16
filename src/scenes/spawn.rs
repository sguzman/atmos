use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::scenes::{
    config::{
        default_circle_color_name, default_circle_radius, default_circle_rgb, default_color_name,
        default_color_rgb, ActiveScene, CameraConfig, CircleConfig, CubeConfig, InputConfig,
        LightConfig, LightEntry, LightOverrides, PillarComboConfig, RectangleConfig,
        RectangleOverrides, SunConfig,
    },
    input::{apply_camera_input, resolve_camera_input_config, SceneCamera, SceneInputConfig},
    loaders::{
        load_camera_config, load_circle_config, load_cube_config, load_input_config,
        load_light_config, load_pillar_combo_config, load_rectangle_config, load_top_light_config,
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
        app.add_systems(
            PostStartup,
            (
                log_lights,
                log_camera,
            )
                .after(setup_scene),
        );
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
    let rectangle_config: RectangleConfig = load_rectangle_config(&active_scene.name);
    let top_light_template: LightConfig = load_top_light_config(&active_scene.name);
    let combo_config: PillarComboConfig = load_pillar_combo_config(&active_scene.name);
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
        &rectangle_config,
        &top_light_template,
        &combo_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    // sun derived from world config
    spawn_sun(world_config.sun.as_ref(), &mut commands, &mut meshes, &mut materials, &active_scene);

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
    rectangle_template: &RectangleConfig,
    top_light_template: &LightConfig,
    combo_template: &PillarComboConfig,
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
            path if path.ends_with("rectangle.toml") => spawn_rectangle(
                entity,
                rectangle_template,
                commands,
                meshes,
                materials,
                active_scene,
            ),
            path if path.ends_with("top_light.toml") => spawn_light_entity(
                entity,
                top_light_template,
                commands,
                active_scene,
            ),
            path if path.ends_with("pillar_with_light.toml") => spawn_pillar_with_light(
                entity,
                rectangle_template,
                top_light_template,
                combo_template,
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
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn spawn_rectangle(
    placement: &EntityPlacement,
    template: &RectangleConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let mut effective = template.clone();
    if let Some(overrides) = &placement.rectangle {
        if let Some(color) = &overrides.color {
            effective.color = color.clone();
        }
        if let Some(dimensions) = &overrides.dimensions {
            effective.dimensions = dimensions.clone();
        }
    }

    let rect_rgb =
        crate::scenes::config::parse_color(&effective.color).unwrap_or_else(|| {
            warn!(
                "Falling back to default color '{}' for rectangle in scene '{}'.",
                default_color_name(),
                active_scene.name
            );
            default_color_rgb()
        });
    let rect_material =
        materials.add(Color::srgb_u8(rect_rgb[0], rect_rgb[1], rect_rgb[2]));

    let rect_rotation = Quat::from_euler(
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
                .unwrap_or_else(|| effective.name.clone()),
        ),
        Mesh3d(meshes.add(Cuboid::new(
            effective.dimensions.width,
            effective.dimensions.height,
            effective.dimensions.depth,
        ))),
        MeshMaterial3d(rect_material),
        Transform::from_xyz(
            placement.transform.position.x,
            placement.transform.position.y,
            placement.transform.position.z,
        )
        .with_rotation(rect_rotation)
        .with_scale(Vec3::splat(placement.transform.scale)),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn spawn_light_entity(
    placement: &EntityPlacement,
    template: &LightConfig,
    commands: &mut Commands,
    active_scene: &ActiveScene,
) {
    // use first light entry from template for simple entity
    let base = template.lights.first().cloned().unwrap_or_else(LightEntry::point_default);
    let merged = merge_light(base, placement.light.as_ref(), None);
    let color = crate::scenes::config::parse_color(&merged.color).unwrap_or([255, 255, 255]);
    let color = Color::srgb_u8(color[0], color[1], color[2]);

    commands.spawn((
        Name::new(
            placement
                .name_override
                .clone()
                .unwrap_or_else(|| template.lights.first().map(|_| "light".to_string()).unwrap_or_default()),
        ),
        PointLight {
            intensity: merged.intensity,
            range: merged.range.unwrap_or(20.0),
            shadows_enabled: merged.shadows,
            color,
            radius: merged.radius.unwrap_or(0.0),
            ..default()
        },
        Transform::from_xyz(
            placement.transform.position.x + merged.offset.x,
            placement.transform.position.y + merged.offset.y,
            placement.transform.position.z + merged.offset.z,
        ),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
    info!(
        "Spawned standalone light '{}' in scene '{}'.",
        placement
            .name_override
            .clone()
            .unwrap_or_else(|| "light".to_string()),
        active_scene.name
    );
}

fn merge_light(
    mut base: LightEntry,
    overrides: Option<&LightOverrides>,
    combo_overrides: Option<&LightOverrides>,
) -> LightEntry {
    for source in [combo_overrides, overrides] {
        if let Some(ovr) = source {
            if let Some(kind) = ovr.kind {
                base.kind = kind;
            }
            if let Some(color) = &ovr.color {
                base.color = color.clone();
            }
            if let Some(intensity) = ovr.intensity {
                base.intensity = intensity;
            }
            if let Some(range) = ovr.range {
                base.range = Some(range);
            }
            if let Some(shadows) = ovr.shadows {
                base.shadows = shadows;
            }
            if let Some(radius) = ovr.radius {
                base.radius = Some(radius);
            }
            if let Some(offset) = &ovr.offset {
                base.offset = offset.clone();
            }
        }
    }
    base
}

fn merge_rectangle(
    mut base: RectangleConfig,
    combo_overrides: &RectangleOverrides,
    world_overrides: Option<&RectangleOverrides>,
) -> RectangleConfig {
    for ovr in [Some(combo_overrides), world_overrides] {
        if let Some(ovr) = ovr {
            if let Some(color) = &ovr.color {
                base.color = color.clone();
            }
            if let Some(dimensions) = &ovr.dimensions {
                base.dimensions = dimensions.clone();
            }
        }
    }
    base
}

fn spawn_pillar_with_light(
    placement: &EntityPlacement,
    rectangle_template: &RectangleConfig,
    light_template: &LightConfig,
    combo_template: &PillarComboConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    // allow template override in combo/world
    let rect_template_to_use = if let Some(path) = combo_template.rectangle.template.as_ref() {
        if path.ends_with("rectangle.toml") {
            // For now we only have one rectangle template per scene
            rectangle_template.clone()
        } else {
            rectangle_template.clone()
        }
    } else {
        rectangle_template.clone()
    };

    let world_rect_override = placement.rectangle.as_ref();
    let rect_effective = merge_rectangle(
        rect_template_to_use,
        &combo_template.rectangle,
        world_rect_override,
    );

    let rect_rgb = crate::scenes::config::parse_color(&rect_effective.color).unwrap_or_else(|| {
        warn!(
            "Falling back to default color '{}' for pillar body in scene '{}'.",
            default_color_name(),
            active_scene.name
        );
        default_color_rgb()
    });
    let rect_material =
        materials.add(Color::srgb_u8(rect_rgb[0], rect_rgb[1], rect_rgb[2]));

    let rect_rotation = Quat::from_euler(
        EulerRot::XYZ,
        placement.transform.rotation.roll.to_radians(),
        placement.transform.rotation.pitch.to_radians(),
        placement.transform.rotation.yaw.to_radians(),
    );

    let rect_scale = placement.transform.scale;
    let rect_transform = Transform::from_xyz(
        placement.transform.position.x,
        placement.transform.position.y,
        placement.transform.position.z,
    )
    .with_rotation(rect_rotation)
    .with_scale(Vec3::splat(rect_scale));

    let body_name = format!(
        "{}_body",
        placement
            .name_override
            .clone()
            .unwrap_or_else(|| combo_template.name.clone())
    );
    commands.spawn((
        Name::new(body_name),
        Mesh3d(meshes.add(Cuboid::new(
            rect_effective.dimensions.width,
            rect_effective.dimensions.height,
            rect_effective.dimensions.depth,
        ))),
        MeshMaterial3d(rect_material),
        rect_transform,
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));

    // Light
    let base_light = light_template
        .lights
        .first()
        .cloned()
        .unwrap_or_else(LightEntry::point_default);
    let merged_light = merge_light(
        base_light,
        placement.light.as_ref(),
        Some(&combo_template.light),
    );
    let light_color = crate::scenes::config::parse_color(&merged_light.color)
        .unwrap_or([255, 255, 255]);
    let light_color = Color::srgb_u8(light_color[0], light_color[1], light_color[2]);

    // Position light on top center plus offsets, respecting rotation/scale
    let top_local = Vec3::new(
        merged_light.offset.x,
        rect_effective.dimensions.height * 0.5 + merged_light.offset.y,
        merged_light.offset.z,
    );
    let light_position_world = rect_transform.transform_point(top_local);

    commands.spawn((
        Name::new(
            placement
                .name_override
                .clone()
                .unwrap_or_else(|| combo_template.name.clone()),
        ),
        PointLight {
            intensity: merged_light.intensity,
            range: merged_light.range.unwrap_or(20.0),
            shadows_enabled: merged_light.shadows,
            color: light_color,
            radius: merged_light.radius.unwrap_or(0.0),
            ..default()
        },
        Transform::from_translation(light_position_world),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn spawn_sun(
    sun: Option<&SunConfig>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    _active_scene: &ActiveScene,
) {
    let Some(sun) = sun else {
        return;
    };

    let fraction = (sun.time.rem_euclid(24.0)) / 24.0;
    let elevation = (std::f32::consts::PI * fraction).sin().max(0.0); // noon highest
    let dir = Vec3::new(0.0, -(0.1 + elevation), -1.0).normalize();
    let sun_color_rgb = crate::scenes::config::parse_color(&sun.color).unwrap_or([255, 255, 255]);
    let sun_color = Color::srgb_u8(sun_color_rgb[0], sun_color_rgb[1], sun_color_rgb[2]);

    // Directional light pointing along dir
    commands.spawn((
        DirectionalLight {
            illuminance: sun.brightness,
            shadows_enabled: false,
            color: sun_color,
            ..default()
        },
        Transform::from_translation(-dir * sun.distance).looking_at(Vec3::ZERO, Vec3::Y),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));

    // Visual sun disc
    let sun_material = materials.add(StandardMaterial {
        base_color: sun_color,
        emissive: sun_color.into(),
        unlit: true,
        ..default()
    });
    commands.spawn((
        Name::new("sun_sphere"),
        Mesh3d(meshes.add(Sphere::new(sun.size))),
        MeshMaterial3d(sun_material),
        Transform::from_translation(-dir * sun.distance),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
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
                    Transform::from_xyz(
                        light.position.x,
                        light.position.y,
                        light.position.z,
                    ),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
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
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
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
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn log_lights(
    point_lights: Query<(&PointLight, &GlobalTransform, &Visibility, &ViewVisibility)>,
    dir_lights: Query<(&DirectionalLight, &GlobalTransform, &Visibility, &ViewVisibility)>,
    ambient: Option<Res<AmbientLight>>,
) {
    info!("Lights present: {} point, {} directional, ambient: {:?}", point_lights.iter().len(), dir_lights.iter().len(), ambient.as_ref().map(|a| (a.color, a.brightness)));
    for (idx, (light, transform, vis, view_vis)) in point_lights.iter().enumerate() {
        info!(
            "Point light #{idx}: intensity={}, range={}, shadows={}, pos={:?}, visibility={:?}, view_visible={}",
            light.intensity,
            light.range,
            light.shadows_enabled,
            transform.translation(),
            vis,
            view_vis.get()
        );
    }
    for (idx, (light, transform, vis, view_vis)) in dir_lights.iter().enumerate() {
        info!(
            "Directional light #{idx}: illuminance={}, shadows={}, dir={:?}, visibility={:?}, view_visible={}",
            light.illuminance,
            light.shadows_enabled,
            transform.forward(),
            vis,
            view_vis.get()
        );
    }
}

fn log_camera(cameras: Query<(&Name, &Transform), With<Camera3d>>) {
    for (name, transform) in cameras.iter() {
        info!(
            "Camera '{}' at {:?} looking {:?}",
            name,
            transform.translation,
            transform.forward()
        );
    }
}
