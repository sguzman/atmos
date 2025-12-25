use bevy::{
    log::warn,
    prelude::*,
};
use bevy_rapier3d::prelude::{AdditionalMassProperties, Collider, Friction, Restitution, RigidBody};

use crate::scenes::config::{
    default_color_name, default_color_rgb, ActiveScene, RectangleConfig, RectangleStackConfig,
};
use crate::scenes::world::EntityPlacement;

pub(super) fn spawn_rectangle_stack(
    placement: &EntityPlacement,
    rectangle_template: &RectangleConfig,
    stack_template: &RectangleStackConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let mut rect_effective = rectangle_template.clone();
    if let Some(color) = &stack_template.rectangle.color {
        rect_effective.color = color.clone();
    }
    if let Some(dimensions) = &stack_template.rectangle.dimensions {
        rect_effective.dimensions = dimensions.clone();
    }

    let rect_rgb =
        crate::scenes::config::parse_color(&rect_effective.color).unwrap_or_else(|| {
            warn!(
                "Falling back to default color '{}' for rectangle stack in scene '{}'.",
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
    let base_transform = Transform::from_xyz(
        placement.transform.position.x,
        placement.transform.position.y,
        placement.transform.position.z,
    )
    .with_rotation(rect_rotation)
    .with_scale(Vec3::splat(rect_scale));

    let spacing = Vec3::new(
        stack_template.spacing.x * rect_scale,
        stack_template.spacing.y * rect_scale,
        stack_template.spacing.z * rect_scale,
    );
    let start_offset = Vec3::new(
        stack_template.start_offset.x * rect_scale,
        stack_template.start_offset.y * rect_scale,
        stack_template.start_offset.z * rect_scale,
    );

    let base_name = placement
        .name_override
        .clone()
        .unwrap_or_else(|| stack_template.name.clone());

    for i in 0..stack_template.count {
        let offset = start_offset + spacing * (i as f32);
        let world_pos = base_transform.transform_point(offset);

        let mut entity = commands.spawn((
            Name::new(format!("{base_name}_{}", i + 1)),
            Mesh3d(meshes.add(Cuboid::new(
                rect_effective.dimensions.width,
                rect_effective.dimensions.height,
                rect_effective.dimensions.depth,
            ))),
            MeshMaterial3d(rect_material.clone()),
            Transform::from_translation(world_pos)
                .with_rotation(rect_rotation)
                .with_scale(Vec3::splat(rect_scale)),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));

        if rect_effective.physics.enabled {
            let half_extents = Vec3::new(
                rect_effective.dimensions.width * 0.5,
                rect_effective.dimensions.height * 0.5,
                rect_effective.dimensions.depth * 0.5,
            );
            let rigid_body = resolve_rigid_body(&rect_effective.physics.body_type);
            entity.insert((
                rigid_body,
                Collider::cuboid(half_extents.x, half_extents.y, half_extents.z),
                Restitution::coefficient(rect_effective.physics.restitution),
                Friction::coefficient(rect_effective.physics.friction),
            ));
            if matches!(rigid_body, RigidBody::Dynamic) && rect_effective.physics.mass > 0.0 {
                entity.insert(AdditionalMassProperties::Mass(rect_effective.physics.mass));
            }
        }
    }
}

fn resolve_rigid_body(body_type: &str) -> RigidBody {
    match body_type.trim().to_ascii_lowercase().as_str() {
        "fixed" | "static" => RigidBody::Fixed,
        "kinematic_position" | "kinematic_position_based" => {
            RigidBody::KinematicPositionBased
        }
        "kinematic_velocity" | "kinematic_velocity_based" => {
            RigidBody::KinematicVelocityBased
        }
        _ => RigidBody::Dynamic,
    }
}
