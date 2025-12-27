use bevy::{
    log::{info, warn},
    prelude::*,
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, Friction, Restitution, RigidBody,
};

use crate::scenes::bounds::DespawnOutsideBounds;
use crate::scenes::config::{
    default_circle_color_name, default_circle_rgb, default_color_name, default_color_rgb,
    parse_color, ActiveScene, EntityTransformConfig, MaterialConfig, PhysicsConfig, ShapeConfig,
    ShapeKind,
};

use super::material::resolve_material;

pub(in crate::scenes::spawn) fn spawn_shape_instance(
    name: &str,
    shape: &ShapeConfig,
    material: Option<&MaterialConfig>,
    physics: Option<&PhysicsConfig>,
    transform: &EntityTransformConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) -> Entity {
    let rotation = Quat::from_euler(
        EulerRot::XYZ,
        transform.rotation.roll.to_radians(),
        transform.rotation.pitch.to_radians(),
        transform.rotation.yaw.to_radians(),
    );

    let material_handle = resolve_material(
        shape,
        material,
        materials,
        asset_server,
        active_scene,
    );

    let entity_id = match shape.kind {
        ShapeKind::Box => {
            let dimensions = shape.dimensions.as_ref().cloned().unwrap_or_default();
            let half_extents = Vec3::new(
                dimensions.width * 0.5,
                dimensions.height * 0.5,
                dimensions.depth * 0.5,
            );
            let mut entity = commands.spawn((
                Name::new(name.to_string()),
                Mesh3d(meshes.add(Cuboid::new(
                    dimensions.width,
                    dimensions.height,
                    dimensions.depth,
                ))),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(
                    transform.position.x,
                    transform.position.y,
                    transform.position.z,
                )
                .with_rotation(rotation)
                .with_scale(Vec3::splat(transform.scale)),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));

            if let Some(physics) = physics {
                if physics.enabled {
                    let rigid_body = resolve_rigid_body(&physics.body_type);
                    entity.insert((
                        rigid_body,
                        Collider::cuboid(half_extents.x, half_extents.y, half_extents.z),
                        Restitution::coefficient(physics.restitution),
                        Friction::coefficient(physics.friction),
                        DespawnOutsideBounds,
                    ));
                    if matches!(rigid_body, RigidBody::Dynamic) && physics.mass > 0.0 {
                        entity.insert(AdditionalMassProperties::Mass(physics.mass));
                    }
                }
            }
            entity.id()
        }
        ShapeKind::Sphere => {
            let radius = shape.radius.unwrap_or(0.5);
            let mut entity = commands.spawn((
                Name::new(name.to_string()),
                Mesh3d(meshes.add(Sphere::new(radius))),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(
                    transform.position.x,
                    transform.position.y,
                    transform.position.z,
                )
                .with_rotation(rotation)
                .with_scale(Vec3::splat(transform.scale)),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));

            if let Some(physics) = physics {
                if physics.enabled {
                    let rigid_body = resolve_rigid_body(&physics.body_type);
                    entity.insert((
                        rigid_body,
                        Collider::ball(radius),
                        Restitution::coefficient(physics.restitution),
                        Friction::coefficient(physics.friction),
                        DespawnOutsideBounds,
                    ));
                    if matches!(rigid_body, RigidBody::Dynamic) && physics.mass > 0.0 {
                        entity.insert(AdditionalMassProperties::Mass(physics.mass));
                    }
                }
            }
            entity.id()
        }
        ShapeKind::Circle => {
            let radius = shape.radius.unwrap_or(4.0);
            let collider_thickness = 0.2;
            let mut entity = commands.spawn((
                Name::new(name.to_string()),
                Mesh3d(meshes.add(Circle::new(radius))),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(
                    transform.position.x,
                    transform.position.y,
                    transform.position.z,
                )
                .with_rotation(rotation)
                .with_scale(Vec3::splat(transform.scale)),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));

            if let Some(physics) = physics {
                if physics.enabled {
                    let rigid_body = resolve_rigid_body(&physics.body_type);
                    entity.insert((rigid_body, DespawnOutsideBounds));
                    if matches!(rigid_body, RigidBody::Dynamic) && physics.mass > 0.0 {
                        entity.insert(AdditionalMassProperties::Mass(physics.mass));
                    }

                    let collider_rotation = rotation.inverse();
                    entity.with_children(|parent| {
                        parent.spawn((
                            Transform::from_rotation(collider_rotation),
                            Collider::cylinder(collider_thickness * 0.5, radius),
                            Restitution::coefficient(physics.restitution),
                            Friction::coefficient(physics.friction),
                        ));
                    });
                }
            }
            entity.id()
        }
    };

    if physics.is_none() {
        info!("Spawned shape '{}' in scene '{}'.", name, active_scene.name);
    }

    entity_id
}

pub(in crate::scenes::spawn) fn resolve_shape_color(
    shape: &ShapeConfig,
    active_scene: &ActiveScene,
) -> [u8; 3] {
    match shape.color.as_deref() {
        Some(color) => parse_color(color).unwrap_or_else(|| fallback_color(shape.kind, active_scene)),
        None => fallback_color(shape.kind, active_scene),
    }
}

fn fallback_color(kind: ShapeKind, active_scene: &ActiveScene) -> [u8; 3] {
    match kind {
        ShapeKind::Circle => {
            warn!(
                "Falling back to default color '{}' for circle in scene '{}'.",
                default_circle_color_name(),
                active_scene.name
            );
            default_circle_rgb()
        }
        ShapeKind::Sphere => [255, 165, 0],
        ShapeKind::Box => {
            warn!(
                "Falling back to default color '{}' for box in scene '{}'.",
                default_color_name(),
                active_scene.name
            );
            default_color_rgb()
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
