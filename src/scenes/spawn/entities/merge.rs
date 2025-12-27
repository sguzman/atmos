use crate::scenes::config::{
    EntityTransformConfig, LightComponent, LightOverridesConfig, PhysicsConfig, PhysicsOverrides,
    ShapeConfig, ShapeOverrides, TransformOverrides,
};

pub(in crate::scenes::spawn) fn merge_shape(
    base: &ShapeConfig,
    overrides: Option<&ShapeOverrides>,
) -> ShapeConfig {
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

pub(in crate::scenes::spawn) fn merge_physics(
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

pub(in crate::scenes::spawn) fn merge_light(
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

pub(in crate::scenes::spawn) fn apply_transform_additive(
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

pub(in crate::scenes::spawn) fn apply_translation(
    mut base: EntityTransformConfig,
    offset: &crate::scenes::config::Vec3Config,
) -> EntityTransformConfig {
    base.position.x += offset.x;
    base.position.y += offset.y;
    base.position.z += offset.z;
    base
}
