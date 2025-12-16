use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::scenes::config::{
    default_circle_color_name, default_circle_radius, default_circle_rgb, default_color_name,
    default_color_rgb, ActiveScene, CircleConfig, CubeConfig, RectangleConfig,
};
use crate::scenes::world::EntityPlacement;

pub(super) fn spawn_circle(
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

pub(super) fn spawn_rectangle(
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

pub(super) fn spawn_cube(
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
