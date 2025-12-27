use bevy::prelude::*;

use crate::scenes::config::{
    ActiveScene, EntityOverrides, EntityTemplate, EntityTransformConfig, TransformOverrides,
};

use super::merge::{merge_light, merge_physics, merge_shape};
use super::{spawn_light_component, spawn_shape_instance};

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
