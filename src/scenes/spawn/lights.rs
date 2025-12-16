use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::scenes::config::{LightConfig, LightEntry, LightKind, LightOverrides};
use crate::scenes::world::EntityPlacement;

pub(super) fn spawn_light_entity(
    placement: &EntityPlacement,
    template: &LightConfig,
    commands: &mut Commands,
    active_scene: &crate::scenes::config::ActiveScene,
) {
    // use first light entry from template for simple entity
    let base = template
        .lights
        .first()
        .cloned()
        .unwrap_or_else(LightEntry::point_default);
    let merged = merge_light(base, placement.light.as_ref(), None);
    let color = crate::scenes::config::parse_color(&merged.color).unwrap_or([255, 255, 255]);
    let color = Color::srgb_u8(color[0], color[1], color[2]);

    commands.spawn((
        Name::new(
            placement
                .name_override
                .clone()
                .unwrap_or_else(|| {
                    template
                        .lights
                        .first()
                        .map(|_| "light".to_string())
                        .unwrap_or_default()
                }),
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

pub(super) fn merge_light(
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

pub(super) fn spawn_lights(light_config: &LightConfig, commands: &mut Commands) {
    let mut ambient_set = false;
    for light in &light_config.lights {
        let color = crate::scenes::config::parse_color(&light.color).unwrap_or([255, 255, 255]);
        let color = Color::srgb_u8(color[0], color[1], color[2]);
        match light.kind {
            LightKind::Ambient => {
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
            LightKind::Point => {
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
            LightKind::Directional => {
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
