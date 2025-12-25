use bevy::{
    log::warn,
    prelude::*,
};

use crate::scenes::config::{LightEntry, LightKind};

pub(super) fn spawn_lights(lights: &[LightEntry], commands: &mut Commands) {
    let mut ambient_set = false;
    for light in lights {
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
