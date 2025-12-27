use bevy::{
    log::warn,
    prelude::*,
};
use std::collections::HashMap;

use crate::scenes::config::{
    ActiveScene, ComboPart, ComboPhysics, ComboStackConfig, ComboTemplate, EntityOverrides,
    EntityTemplate, TransformOverrides, Vec3Config,
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
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) {
    let combo_name = name_override
        .cloned()
        .unwrap_or_else(|| combo.name.clone());
    let shared_physics = combo
        .physics
        .as_ref()
        .map(|physics| physics.shared)
        .unwrap_or(false);

    let fallback_stack = default_stack();
    let (stack_config, stack_count) = match combo.stack.as_ref() {
        Some(stack) => (stack, stack.count.max(1)),
        None => (&fallback_stack, 1),
    };

    let mut loaded_parts = Vec::new();
    for part in &combo.parts {
        let Some(template) =
            load_entity_template_from_path(&active_scene.name, &part.template)
        else {
            warn!(
                "Failed to load combo part template '{}' in scene '{}'; skipping.",
                part.template, active_scene.name
            );
            continue;
        };
        let part_name = part
            .name_override
            .clone()
            .unwrap_or_else(|| template.name.clone());
        loaded_parts.push(LoadedPart {
            part,
            template,
            part_name,
        });
    }

    let root_name = if shared_physics {
        resolve_physics_root(&combo.physics, &loaded_parts)
    } else {
        None
    };

    for i in 0..stack_count {
        let instance_offset = stack_instance_offset(stack_config, i);
        let instance_suffix = if stack_count > 1 {
            format!("_{}", i + 1)
        } else {
            String::new()
        };

        let mut spawned: HashMap<String, Entity> = HashMap::new();

        if let Some(root) = root_name.as_ref() {
            if let Some(root_part) =
                loaded_parts.iter().find(|part| &part.part_name == root)
            {
                let entity = spawn_combo_part(
                    &combo_name,
                    &instance_suffix,
                    root_part,
                    combo.overrides.as_ref(),
                    placement_transform,
                    placement_overrides,
                    &instance_offset,
                    shared_physics,
                    true,
                    &mut spawned,
                    commands,
                    meshes,
                    materials,
                    asset_server,
                    active_scene,
                );
                if let Some(entity) = entity {
                    spawned.insert(root_part.part_name.clone(), entity);
                }
            }
        }

        for part in &loaded_parts {
            if Some(&part.part_name) == root_name.as_ref() {
                continue;
            }
            let entity = spawn_combo_part(
                &combo_name,
                &instance_suffix,
                part,
                combo.overrides.as_ref(),
                placement_transform,
                placement_overrides,
                &instance_offset,
                shared_physics,
                false,
                &mut spawned,
                commands,
                meshes,
                materials,
                asset_server,
                active_scene,
            );
            if let Some(entity) = entity {
                spawned.insert(part.part_name.clone(), entity);
            }
        }
    }
}

struct LoadedPart<'a> {
    part: &'a ComboPart,
    template: EntityTemplate,
    part_name: String,
}

fn resolve_physics_root(
    physics: &Option<ComboPhysics>,
    parts: &[LoadedPart<'_>],
) -> Option<String> {
    let Some(physics) = physics else {
        return None;
    };
    if let Some(root) = &physics.root {
        return Some(root.clone());
    }
    if let Some(part) = parts.iter().find(|part| part.part.physics_root) {
        return Some(part.part_name.clone());
    }
    parts.first().map(|part| part.part_name.clone())
}

fn spawn_combo_part(
    combo_name: &str,
    instance_suffix: &str,
    part: &LoadedPart<'_>,
    combo_overrides: Option<&EntityOverrides>,
    placement_transform: &TransformOverrides,
    placement_overrides: &EntityOverrides,
    instance_offset: &Vec3Config,
    shared_physics: bool,
    is_root: bool,
    spawned: &mut HashMap<String, Entity>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    active_scene: &ActiveScene,
) -> Option<Entity> {
    let mut transform = part.template.transform.clone();
    if let Some(part_transform) = &part.part.transform {
        transform = apply_transform_additive(transform, part_transform);
    }
    let attach = part.part.attach.as_ref();
    let attach_target = attach.and_then(|attach| spawned.get(&attach.target).copied());
    if let Some(attach) = attach {
        if attach_target.is_some() {
            transform = apply_translation(transform, &attach.offset);
        } else {
            transform = apply_translation(transform, instance_offset);
            transform = apply_transform_additive(transform, placement_transform);
        }
    } else {
        transform = apply_translation(transform, instance_offset);
        transform = apply_transform_additive(transform, placement_transform);
    }

    let mut shape = part.template.shape.as_ref().cloned();
    let material = part.template.material.as_ref();
    let mut physics = part.template.physics.as_ref().cloned();
    let mut light = part.template.light.as_ref().cloned();

    for overrides in [
        combo_overrides,
        part.part.overrides.as_ref(),
        Some(placement_overrides),
    ] {
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

    let full_name = format!("{combo_name}{instance_suffix}_{}", part.part_name);
    let physics_allowed = !shared_physics || is_root;

    let mut entity_id = None;
    if let Some(shape) = shape {
        let entity = spawn_shape_instance(
            &full_name,
            &shape,
            material,
            if physics_allowed { physics.as_ref() } else { None },
            &transform,
            commands,
            meshes,
            materials,
            asset_server,
            active_scene,
        );
        entity_id = Some(entity);
    }

    if let Some(light) = light {
        let light_entity = spawn_light_component(
            &full_name,
            &light,
            &transform,
            commands,
            active_scene,
        );
        if entity_id.is_none() {
            entity_id = light_entity;
        }
    }

    if let Some(attach) = attach {
        if let Some(target) = attach_target {
            if let Some(child) = entity_id {
                commands.entity(target).add_child(child);
            }
        } else {
            warn!(
                "Attach target '{}' not found for combo '{}'; leaving '{}' unparented.",
                attach.target, combo_name, part.part_name
            );
        }
    }

    entity_id
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
