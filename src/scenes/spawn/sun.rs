use bevy::prelude::*;

use crate::scenes::config::SunConfig;

pub(super) fn spawn_sun(
    sun: Option<&SunConfig>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    _active_scene: &crate::scenes::config::ActiveScene,
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
