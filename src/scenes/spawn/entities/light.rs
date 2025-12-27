use bevy::{log::warn, prelude::*};

use crate::scenes::config::{parse_color, ActiveScene, LightComponent, LightKind, LightEntry};

pub(in crate::scenes::spawn) fn spawn_light_component(
    name: &str,
    light: &LightComponent,
    transform: &crate::scenes::config::EntityTransformConfig,
    commands: &mut Commands,
    active_scene: &ActiveScene,
) -> Option<Entity> {
    let mut entry = LightEntry::point_default();
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
