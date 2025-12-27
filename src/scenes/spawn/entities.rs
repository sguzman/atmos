use bevy::{
    log::{info, warn},
    prelude::*,
};
use bevy::image::ImageLoaderSettings;
use bevy::render::alpha::AlphaMode;
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, Friction, Restitution, RigidBody,
};

use crate::scenes::bounds::DespawnOutsideBounds;
use crate::scenes::config::{
    default_circle_color_name, default_circle_rgb, default_color_name, default_color_rgb,
    parse_color, ActiveScene, EntityOverrides, EntityTemplate, EntityTransformConfig,
    LightComponent, LightKind, LightOverridesConfig, MaterialConfig, PhysicsConfig,
    PhysicsOverrides, ShapeConfig, ShapeKind, ShapeOverrides, TransformOverrides,
};

pub fn spawn_entity_from_template(
    template: &EntityTemplate,
    overrides: &EntityOverrides,
    placement_transform: &TransformOverrides,
    name_override: Option<&String>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) {
    let transform = merge_transform(&template.transform, placement_transform, None);
    let shape = template
        .shape
        .as_ref()
        .map(|shape| merge_shape(shape, overrides.shape.as_ref()));
    let material = template.material.as_ref();
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
            material,
            physics.as_ref(),
            &transform,
            commands,
            meshes,
            materials,
            asset_server,
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

fn resolve_material(
    shape: &ShapeConfig,
    material: Option<&MaterialConfig>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) -> Handle<StandardMaterial> {
    let shape_color = resolve_shape_color(shape, active_scene);
    let base_color_fallback = Color::srgb_u8(shape_color[0], shape_color[1], shape_color[2]);
    let mut resolved = if let Some(material) = material {
        resolve_material_from_config(material, base_color_fallback, asset_server)
    } else {
        let mut material = StandardMaterial::default();
        material.base_color = base_color_fallback;
        material
    };

    if resolved.alpha_mode == AlphaMode::Opaque {
        if resolved.base_color.to_srgba().alpha < 1.0 {
            resolved.alpha_mode = AlphaMode::Blend;
        }
    }

    materials.add(resolved)
}

fn resolve_material_from_config(
    config: &MaterialConfig,
    base_color_fallback: Color,
    asset_server: &AssetServer,
) -> StandardMaterial {
    let mut material = match config
        .preset
        .as_deref()
        .map(|preset| preset.trim().to_ascii_lowercase().replace('-', "_"))
        .as_deref()
    {
        Some("wood") | Some("wooden") => preset_wood(),
        Some("metal") | Some("metallic") => preset_metal(),
        Some("marble") => preset_marble(),
        Some("stone") => preset_stone(),
        Some("glass") => preset_glass(),
        _ => StandardMaterial::default(),
    };

    if let Some(color) = config.base_color.as_deref().and_then(parse_color) {
        material.base_color = Color::srgb_u8(color[0], color[1], color[2]);
    } else if config.preset.is_none() {
        material.base_color = base_color_fallback;
    }

    if let Some(opacity) = config.opacity {
        let mut color = material.base_color;
        color.set_alpha(opacity.clamp(0.0, 1.0));
        material.base_color = color;
    }

    if let Some(path) = config.base_color_texture.as_deref() {
        material.base_color_texture = Some(load_texture(asset_server, path, true));
    }

    if let Some(metallic) = config.metallic {
        material.metallic = metallic.clamp(0.0, 1.0);
    }
    if let Some(roughness) = config.roughness {
        material.perceptual_roughness = roughness.clamp(0.0, 1.0);
    }
    if let Some(reflectance) = config.reflectance {
        material.reflectance = reflectance.clamp(0.0, 1.0);
    }
    if let Some(color) = config.specular_tint.as_deref().and_then(parse_color) {
        material.specular_tint = Color::srgb_u8(color[0], color[1], color[2]);
    }

    if let Some(color) = config.emissive_color.as_deref().and_then(parse_color) {
        let intensity = config.emissive_intensity.unwrap_or(1.0);
        let base = Color::srgb_u8(color[0], color[1], color[2]);
        material.emissive = base.to_linear() * intensity;
    } else if let Some(intensity) = config.emissive_intensity {
        material.emissive = Color::WHITE.to_linear() * intensity;
    }

    if let Some(path) = config.emissive_texture.as_deref() {
        material.emissive_texture = Some(load_texture(asset_server, path, true));
    }

    if let Some(path) = config.normal_map.as_deref() {
        material.normal_map_texture = Some(load_texture(asset_server, path, false));
    }
    if let Some(flip) = config.flip_normal_map_y {
        material.flip_normal_map_y = flip;
    }
    if let Some(path) = config.metallic_roughness_texture.as_deref() {
        material.metallic_roughness_texture =
            Some(load_texture(asset_server, path, false));
    }
    if let Some(path) = config.occlusion_texture.as_deref() {
        material.occlusion_texture = Some(load_texture(asset_server, path, false));
    }

    if let Some(alpha_mode) = config.alpha_mode.as_deref() {
        material.alpha_mode = match alpha_mode.trim().to_ascii_lowercase().as_str() {
            "blend" => AlphaMode::Blend,
            "premultiplied" => AlphaMode::Premultiplied,
            "add" => AlphaMode::Add,
            "multiply" => AlphaMode::Multiply,
            "mask" => AlphaMode::Mask(config.alpha_cutoff.unwrap_or(0.5)),
            _ => AlphaMode::Opaque,
        };
    } else if config.opacity.unwrap_or(1.0) < 1.0 {
        material.alpha_mode = AlphaMode::Blend;
    }

    if let Some(unlit) = config.unlit {
        material.unlit = unlit;
    }
    if let Some(double_sided) = config.double_sided {
        material.double_sided = double_sided;
    }

    if let Some(clearcoat) = config.clearcoat {
        material.clearcoat = clearcoat.clamp(0.0, 1.0);
    }
    if let Some(roughness) = config.clearcoat_roughness {
        material.clearcoat_perceptual_roughness = roughness.clamp(0.0, 1.0);
    }
    if let Some(ior) = config.ior {
        material.ior = ior;
    }
    if let Some(transmission) = config.specular_transmission {
        material.specular_transmission = transmission.clamp(0.0, 1.0);
    }
    if let Some(transmission) = config.diffuse_transmission {
        material.diffuse_transmission = transmission.clamp(0.0, 1.0);
    }
    if let Some(thickness) = config.thickness {
        material.thickness = thickness.max(0.0);
    }
    if let Some(color) = config.attenuation_color.as_deref().and_then(parse_color) {
        material.attenuation_color = Color::srgb_u8(color[0], color[1], color[2]);
    }
    if let Some(distance) = config.attenuation_distance {
        material.attenuation_distance = distance.max(0.0);
    }

    material
}

fn load_texture(
    asset_server: &AssetServer,
    path: &str,
    is_srgb: bool,
) -> Handle<Image> {
    asset_server.load_with_settings(
        path.to_string(),
        move |settings: &mut ImageLoaderSettings| settings.is_srgb = is_srgb,
    )
}

fn preset_wood() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(140, 96, 64),
        metallic: 0.0,
        perceptual_roughness: 0.7,
        reflectance: 0.5,
        ..default()
    }
}

fn preset_metal() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(180, 180, 190),
        metallic: 1.0,
        perceptual_roughness: 0.2,
        reflectance: 0.9,
        ..default()
    }
}

fn preset_marble() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(235, 232, 225),
        metallic: 0.0,
        perceptual_roughness: 0.25,
        reflectance: 0.6,
        ..default()
    }
}

fn preset_stone() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb_u8(120, 120, 120),
        metallic: 0.0,
        perceptual_roughness: 0.85,
        reflectance: 0.4,
        ..default()
    }
}

fn preset_glass() -> StandardMaterial {
    let mut material = StandardMaterial {
        base_color: Color::srgba_u8(200, 220, 230, 20),
        metallic: 0.0,
        perceptual_roughness: 0.05,
        reflectance: 0.9,
        specular_transmission: 1.0,
        thickness: 0.5,
        ior: 1.5,
        ..default()
    };
    material.alpha_mode = AlphaMode::Blend;
    material
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
