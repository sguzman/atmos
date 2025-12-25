use bevy::{
    log::warn,
    prelude::*,
};

use crate::scenes::config::{
    ActiveScene, ComboStackConfig, ComboTemplate, EntityOverrides, EntityTemplate,
    TransformOverrides, Vec3Config,
};
use crate::scenes::loaders::load_entity_template_from_path;

use super::entities::{
    apply_transform_additive, apply_translation, merge_light, merge_physics, merge_shape,
    spawn_light_component, spawn_shape_instance,
};

pub(super) fn spawn_combo_template(
    combo: &ComboTemplate,
    placement_transform: &TransformOverrides,
    placement_overrides: &EntityOverrides,
    name_override: Option<&String>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let combo_name = name_override
        .cloned()
        .unwrap_or_else(|| combo.name.clone());

    let fallback_stack = default_stack();
    let (stack_config, stack_count) = match combo.stack.as_ref() {
        Some(stack) => (stack, stack.count.max(1)),
        None => (&fallback_stack, 1),
    };

    for i in 0..stack_count {
        let instance_offset = stack_instance_offset(stack_config, i);
        let instance_suffix = if stack_count > 1 {
            format!("_{}", i + 1)
        } else {
            String::new()
        };

        for part in &combo.parts {
            let Some(base_template) =
                load_entity_template_from_path(&active_scene.name, &part.template)
            else {
                warn!(
                    "Failed to load combo part template '{}' in scene '{}'; skipping.",
                    part.template, active_scene.name
                );
                continue;
            };

            spawn_combo_part(
                &combo_name,
                &instance_suffix,
                &base_template,
                combo.overrides.as_ref(),
                part,
                placement_transform,
                placement_overrides,
                &instance_offset,
                commands,
                meshes,
                materials,
                active_scene,
            );
        }
    }
}

fn spawn_combo_part(
    combo_name: &str,
    instance_suffix: &str,
    template: &EntityTemplate,
    combo_overrides: Option<&EntityOverrides>,
    part: &crate::scenes::config::ComboPart,
    placement_transform: &TransformOverrides,
    placement_overrides: &EntityOverrides,
    instance_offset: &Vec3Config,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    active_scene: &ActiveScene,
) {
    let mut transform = template.transform.clone();
    if let Some(part_transform) = &part.transform {
        transform = apply_transform_additive(transform, part_transform);
    }
    transform = apply_translation(transform, instance_offset);
    transform = apply_transform_additive(transform, placement_transform);

    let mut shape = template.shape.as_ref().cloned();
    let mut physics = template.physics.as_ref().cloned();
    let mut light = template.light.as_ref().cloned();

    for overrides in [combo_overrides, part.overrides.as_ref(), Some(placement_overrides)] {
        if let Some(ovr) = overrides {
            if let Some(shape_override) = ovr.shape.as_ref() {
                if let Some(current) = &shape {
                    shape = Some(merge_shape(current, Some(shape_override)));
                }
            }
            if let Some(physics_override) = ovr.physics.as_ref() {
                if let Some(current) = &physics {
                    physics = Some(merge_physics(current, Some(physics_override)));
                }
            }
            if let Some(light_override) = ovr.light.as_ref() {
                if let Some(current) = &light {
                    light = Some(merge_light(current, Some(light_override)));
                }
            }
        }
    }

    let part_name = part
        .name_override
        .clone()
        .unwrap_or_else(|| template.name.clone());
    let full_name = format!("{combo_name}{instance_suffix}_{part_name}");

    if let Some(shape) = shape {
        spawn_shape_instance(
            &full_name,
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
        spawn_light_component(
            &full_name,
            &light,
            &transform,
            commands,
            active_scene,
        );
    }
}

fn stack_instance_offset(stack: &ComboStackConfig, index: u32) -> Vec3Config {
    let step = Vec3Config {
        x: stack.spacing.x * index as f32,
        y: stack.spacing.y * index as f32,
        z: stack.spacing.z * index as f32,
    };
    Vec3Config {
        x: stack.start_offset.x + step.x,
        y: stack.start_offset.y + step.y,
        z: stack.start_offset.z + step.z,
    }
}

fn default_stack() -> ComboStackConfig {
    ComboStackConfig {
        count: 1,
        spacing: Vec3Config::default(),
        start_offset: Vec3Config::default(),
    }
}
