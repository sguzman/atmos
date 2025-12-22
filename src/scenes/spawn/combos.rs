use bevy::{
    log::warn,
    prelude::*,
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, Friction, Restitution, RigidBody,
};

use crate::scenes::config::{
    default_color_name, default_color_rgb, ActiveScene, LightConfig, LightEntry,
    PillarComboConfig, RectangleConfig, RectangleOverrides,
};
use crate::scenes::world::EntityPlacement;

use super::lights::merge_light;

pub(super) fn spawn_pillar_with_light(
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
    let mut entity = commands.spawn((
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

    if rect_effective.physics.enabled {
        let rigid_body = resolve_rigid_body(&rect_effective.physics.body_type);
        let half_extents = Vec3::new(
            rect_effective.dimensions.width * 0.5,
            rect_effective.dimensions.height * 0.5,
            rect_effective.dimensions.depth * 0.5,
        );
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
