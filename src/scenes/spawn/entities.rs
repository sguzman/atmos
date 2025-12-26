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
    parse_color, ActiveScene, EntityOverrides, EntityTemplate, EntityTransformConfig,
    LightComponent, LightKind,
    LightOverridesConfig, PhysicsConfig, PhysicsOverrides, ShapeConfig, ShapeKind, ShapeOverrides,
    TransformOverrides,
};

pub fn spawn_entity_from_template(
    template: &EntityTemplate,
    overrides: &EntityOverrides,
    placement_transform: &TransformOverrides,
    name_override: Option<&String>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let transform = merge_transform(&template.transform, placement_transform, None);
    let shape = template
        .shape
        .as_ref()
        .map(|shape| merge_shape(shape, overrides.shape.as_ref()));
    let physics = template
        .physics
        .as_ref()
        .map(|physics| merge_physics(physics, overrides.physics.as_ref()));
    let light = template
        .light
        .as_ref()
        .map(|light| merge_light(light, overrides.light.as_ref()));
    let base_name = name_override
        .cloned()
        .unwrap_or_else(|| template.name.clone());

    if let Some(shape) = shape {
        spawn_shape_instance(
            &base_name,
            &shape,
            physics.as_ref(),
            &transform,
            commands,
            meshes,
            materials,
            active_scene,
        );
    }

    if let Some(light) = light {
        let _ = spawn_light_component(
            &base_name,
            &light,
            &transform,
            commands,
            active_scene,
        );
    }
}

fn merge_transform(
    base: &EntityTransformConfig,
    placement: &TransformOverrides,
    overrides: Option<&TransformOverrides>,
) -> EntityTransformConfig {
    let mut merged = base.clone();
    for source in [overrides, Some(placement)] {
        if let Some(ovr) = source {
            if let Some(position) = &ovr.position {
                merged.position = position.clone();
            }
            if let Some(rotation) = &ovr.rotation {
                merged.rotation = rotation.clone();
            }
            if let Some(scale) = ovr.scale {
                merged.scale = scale;
            }
        }
    }
    merged
}

pub(super) fn merge_shape(base: &ShapeConfig, overrides: Option<&ShapeOverrides>) -> ShapeConfig {
    let mut merged = base.clone();
    if let Some(ovr) = overrides {
        if let Some(color) = &ovr.color {
            merged.color = Some(color.clone());
        }
        if let Some(dimensions) = &ovr.dimensions {
            merged.dimensions = Some(dimensions.clone());
        }
        if let Some(radius) = ovr.radius {
            merged.radius = Some(radius);
        }
    }
    merged
}

pub(super) fn merge_physics(
    base: &PhysicsConfig,
    overrides: Option<&PhysicsOverrides>,
) -> PhysicsConfig {
    let mut merged = base.clone();
    if let Some(ovr) = overrides {
        if let Some(enabled) = ovr.enabled {
            merged.enabled = enabled;
        }
        if let Some(body_type) = &ovr.body_type {
            merged.body_type = body_type.clone();
        }
        if let Some(mass) = ovr.mass {
            merged.mass = mass;
        }
        if let Some(restitution) = ovr.restitution {
            merged.restitution = restitution;
        }
        if let Some(friction) = ovr.friction {
            merged.friction = friction;
        }
    }
    merged
}

pub(super) fn merge_light(
    base: &LightComponent,
    overrides: Option<&LightOverridesConfig>,
) -> LightComponent {
    let mut merged = base.clone();
    if let Some(ovr) = overrides {
        if let Some(kind) = ovr.kind {
            merged.kind = Some(kind);
        }
        if let Some(color) = &ovr.color {
            merged.color = Some(color.clone());
        }
        if let Some(intensity) = ovr.intensity {
            merged.intensity = Some(intensity);
        }
        if let Some(range) = ovr.range {
            merged.range = Some(range);
        }
        if let Some(shadows) = ovr.shadows {
            merged.shadows = Some(shadows);
        }
        if let Some(radius) = ovr.radius {
            merged.radius = Some(radius);
        }
        if let Some(offset) = &ovr.offset {
            merged.offset = Some(offset.clone());
        }
    }
    merged
}

pub(super) fn spawn_shape_instance(
    name: &str,
    shape: &ShapeConfig,
    physics: Option<&PhysicsConfig>,
    transform: &EntityTransformConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) -> Entity {
    let rotation = Quat::from_euler(
        EulerRot::XYZ,
        transform.rotation.roll.to_radians(),
        transform.rotation.pitch.to_radians(),
        transform.rotation.yaw.to_radians(),
    );

    let color = resolve_shape_color(shape, active_scene);
    let material = materials.add(Color::srgb_u8(color[0], color[1], color[2]));

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
                MeshMaterial3d(material),
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
                MeshMaterial3d(material),
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
                MeshMaterial3d(material),
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

pub(super) fn apply_transform_additive(
    mut base: EntityTransformConfig,
    delta: &TransformOverrides,
) -> EntityTransformConfig {
    if let Some(position) = &delta.position {
        base.position.x += position.x;
        base.position.y += position.y;
        base.position.z += position.z;
    }
    if let Some(rotation) = &delta.rotation {
        base.rotation.roll += rotation.roll;
        base.rotation.pitch += rotation.pitch;
        base.rotation.yaw += rotation.yaw;
    }
    if let Some(scale) = delta.scale {
        base.scale *= scale;
    }
    base
}

pub(super) fn apply_translation(
    mut base: EntityTransformConfig,
    offset: &crate::scenes::config::Vec3Config,
) -> EntityTransformConfig {
    base.position.x += offset.x;
    base.position.y += offset.y;
    base.position.z += offset.z;
    base
}

pub(super) fn spawn_light_component(
    name: &str,
    light: &LightComponent,
    transform: &EntityTransformConfig,
    commands: &mut Commands,
    active_scene: &ActiveScene,
) -> Option<Entity> {
    let mut entry = crate::scenes::config::LightEntry::point_default();
    if let Some(kind) = light.kind {
        entry.kind = kind;
    }
    if let Some(color) = &light.color {
        entry.color = color.clone();
    }
    if let Some(intensity) = light.intensity {
        entry.intensity = intensity;
    }
    if let Some(range) = light.range {
        entry.range = Some(range);
    }
    if let Some(shadows) = light.shadows {
        entry.shadows = shadows;
    }
    if let Some(radius) = light.radius {
        entry.radius = Some(radius);
    }
    if let Some(offset) = &light.offset {
        entry.offset = offset.clone();
    }

    if entry.kind != LightKind::Point {
        warn!(
            "Entity light '{}' in scene '{}' is not a point light; skipping.",
            name, active_scene.name
        );
        return None;
    }

    let color = parse_color(&entry.color).unwrap_or([255, 255, 255]);
    let color = Color::srgb_u8(color[0], color[1], color[2]);
    let base_transform = Transform::from_xyz(
        transform.position.x,
        transform.position.y,
        transform.position.z,
    )
    .with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        transform.rotation.roll.to_radians(),
        transform.rotation.pitch.to_radians(),
        transform.rotation.yaw.to_radians(),
    ))
    .with_scale(Vec3::splat(transform.scale));
    let offset = Vec3::new(entry.offset.x, entry.offset.y, entry.offset.z);
    let world_pos = base_transform.transform_point(offset);

    let entity = commands.spawn((
        Name::new(name.to_string()),
        PointLight {
            intensity: entry.intensity,
            range: entry.range.unwrap_or(20.0),
            shadows_enabled: entry.shadows,
            color,
            radius: entry.radius.unwrap_or(0.0),
            ..default()
        },
        Transform::from_translation(world_pos),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
    Some(entity.id())
}

fn resolve_shape_color(shape: &ShapeConfig, active_scene: &ActiveScene) -> [u8; 3] {
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
