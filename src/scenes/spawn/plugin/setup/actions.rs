use bevy::prelude::*;

use std::collections::HashMap;

use crate::scenes::{
    config::{
        ActionConfig, ActionTriggerConfig, ActionsConfig, ActiveScene, ShapeKind, TriggerMode,
        VolumeShapeKind, VolumeTriggerMode,
    },
    input::{
        ActionStates, FovBinding, GrabHover, GrabState, NoclipState, PauseState,
        ResolvedActionTrigger, ResolvedVolumeTrigger, SceneActionTriggers, SceneFovConfig,
        SceneGrabConfig, SceneGrenadeConfig, SceneJumpConfig, SceneNoclipConfig, ScenePauseConfig,
        SceneReloadConfig, SceneShootConfig, SceneSprintConfig, SceneZoomConfig, SprintState,
        TriggerSource, ZoomState,
        TriggerMode as InputTriggerMode, VolumeShape, VolumeShapeKind as InputVolumeShapeKind,
        VolumeTriggerMode as InputVolumeTriggerMode,
    },
    loaders::{load_entity_template_from_path, ConfigLoad, TomlCache},
    MeshCacheSettings, TomlAsset,
};

pub(crate) fn setup_actions(
    actions_config: &ActionsConfig,
    active_scene: &ActiveScene,
    mesh_cache: &MeshCacheSettings,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
    toml_cache: &mut TomlCache,
) -> Option<bool> {
    let mut actions_by_id = HashMap::new();
    for action in actions_config.actions.iter() {
        actions_by_id.insert(action.id().to_string(), action.clone());
    }

    let mut initial_noclip = None;
    let mut fov_bindings = Vec::new();
    for action in actions_config.actions.iter() {
        match action {
            ActionConfig::Shoot { id, params } => {
                let projectile = match load_entity_template_from_path(
                    &active_scene.name,
                    "entities/sphere.3D.toml",
                    toml_cache,
                    asset_server,
                    toml_assets,
                ) {
                    ConfigLoad::Pending => return None,
                    ConfigLoad::Ready(template) => template,
                };
                let Some(projectile) = projectile else {
                    warn!("Projectile template missing; shoot action disabled.");
                    continue;
                };

                let Some(shape) = projectile.shape.clone() else {
                    warn!("Projectile template has no shape; shoot action disabled.");
                    continue;
                };
                if shape.kind != ShapeKind::Sphere {
                    warn!("Projectile template is not a sphere; shoot action disabled.");
                    continue;
                }
                let color = shape
                    .color
                    .as_deref()
                    .and_then(crate::scenes::config::parse_color)
                    .unwrap_or([255, 165, 0]);
                let sphere_material = materials.add(Color::srgb_u8(color[0], color[1], color[2]));
                let sphere_mesh = crate::scenes::load_or_generate_mesh_handle(
                    mesh_cache,
                    &shape,
                    meshes,
                    asset_server,
                );
                commands.insert_resource(SceneShootConfig {
                    id: id.clone(),
                    action: params.clone(),
                    name: projectile.name.clone(),
                    shape,
                    physics: projectile.physics.clone(),
                    mesh: sphere_mesh,
                    material: sphere_material,
                });
            }
            ActionConfig::Grenade { id, params } => {
                let projectile = match load_entity_template_from_path(
                    &active_scene.name,
                    "entities/sphere.3D.toml",
                    toml_cache,
                    asset_server,
                    toml_assets,
                ) {
                    ConfigLoad::Pending => return None,
                    ConfigLoad::Ready(template) => template,
                };
                let Some(projectile) = projectile else {
                    warn!("Grenade template missing; grenade action disabled.");
                    continue;
                };

                let Some(mut shape) = projectile.shape.clone() else {
                    warn!("Grenade template has no shape; grenade action disabled.");
                    continue;
                };
                if shape.kind != ShapeKind::Sphere {
                    warn!("Grenade template is not a sphere; grenade action disabled.");
                    continue;
                }
                if params.radius > 0.0 {
                    shape.radius = Some(params.radius);
                }
                if !params.color.trim().is_empty() {
                    shape.color = Some(params.color.clone());
                }
                let color = shape
                    .color
                    .as_deref()
                    .and_then(crate::scenes::config::parse_color)
                    .unwrap_or([0, 255, 0]);
                let sphere_material = materials.add(Color::srgb_u8(color[0], color[1], color[2]));
                let sphere_mesh = crate::scenes::load_or_generate_mesh_handle(
                    mesh_cache,
                    &shape,
                    meshes,
                    asset_server,
                );
                commands.insert_resource(SceneGrenadeConfig {
                    id: id.clone(),
                    action: params.clone(),
                    name: projectile.name.clone(),
                    shape,
                    physics: projectile.physics.clone(),
                    mesh: sphere_mesh,
                    material: sphere_material,
                });
            }
            ActionConfig::Sprint { id, params } => {
                commands.insert_resource(SceneSprintConfig {
                    id: id.clone(),
                    action: params.clone(),
                });
                commands.insert_resource(SprintState::default());
            }
            ActionConfig::Zoom { id, params } => {
                commands.insert_resource(SceneZoomConfig {
                    id: id.clone(),
                    action: params.clone(),
                });
                commands.insert_resource(ZoomState::default());
            }
            ActionConfig::Jump { id, params } => {
                commands.insert_resource(SceneJumpConfig {
                    id: id.clone(),
                    action: params.clone(),
                });
            }
            ActionConfig::Noclip { id, params } => {
                let state = NoclipState {
                    active: params.enabled,
                    velocity: Vec3::ZERO,
                    fast: false,
                };
                let speed_toggle_key = crate::scenes::input::resolve_key_or_warn(
                    &params.speed_toggle_key,
                    "noclip speed toggle",
                );
                let up_key =
                    crate::scenes::input::resolve_key_or_warn(&params.up_key, "noclip up");
                let down_key =
                    crate::scenes::input::resolve_key_or_warn(&params.down_key, "noclip down");
                initial_noclip = Some(params.enabled);
                commands.insert_resource(SceneNoclipConfig {
                    id: id.clone(),
                    action: params.clone(),
                    speed_toggle_key,
                    up_key,
                    down_key,
                });
                commands.insert_resource(state);
            }
            ActionConfig::Grab { id, params } => {
                let rgb = crate::scenes::config::parse_color(&params.outline.color)
                    .unwrap_or([0, 255, 255]);
                let outline_color = Color::srgb_u8(rgb[0], rgb[1], rgb[2]);
                commands.insert_resource(SceneGrabConfig {
                    id: id.clone(),
                    action: params.clone(),
                    outline_color,
                });
                commands.insert_resource(GrabState::default());
                commands.insert_resource(GrabHover::default());
            }
            ActionConfig::Reload { id, .. } => {
                commands.insert_resource(SceneReloadConfig { id: id.clone() });
            }
            ActionConfig::Pause { id, params } => {
                commands.insert_resource(ScenePauseConfig {
                    id: id.clone(),
                    action: params.clone(),
                });
                commands.insert_resource(PauseState {
                    active: false,
                    pause_scene: params.pause_scene,
                    overlay: params.overlay.clone(),
                    stored_time_scale: 1.0,
                });
            }
            ActionConfig::Fov { id, params } => {
                fov_bindings.push(FovBinding {
                    action_id: id.clone(),
                    fov_degrees: params.fov_degrees,
                });
            }
            _ => {}
        }
    }

    if !fov_bindings.is_empty() {
        commands.insert_resource(SceneFovConfig {
            bindings: fov_bindings,
        });
    }

    let mut resolved_triggers = Vec::new();
    let mut resolved_volumes = Vec::new();
    for trigger in actions_config.triggers.iter() {
        let action_id = match trigger {
            ActionTriggerConfig::Key { action, .. }
            | ActionTriggerConfig::Mouse { action, .. }
            | ActionTriggerConfig::Volume { action, .. } => action,
        };
        if !actions_by_id.contains_key(action_id) {
            warn!("Action trigger references unknown action id '{action_id}'.");
            continue;
        }
        match trigger {
            ActionTriggerConfig::Key { key, mode, action, .. } => {
                if let Some(trigger) =
                    crate::scenes::input::resolve_key_or_warn(key, "action key")
                {
                    resolved_triggers.push(ResolvedActionTrigger {
                        action: action.clone(),
                        source: TriggerSource::Key(trigger),
                        mode: map_trigger_mode(*mode),
                    });
                }
            }
            ActionTriggerConfig::Mouse { mouse, mode, action, .. } => {
                if let Some(trigger) =
                    crate::scenes::input::resolve_mouse_button_or_warn(mouse, "action mouse")
                {
                    resolved_triggers.push(ResolvedActionTrigger {
                        action: action.clone(),
                        source: TriggerSource::Mouse(trigger),
                        mode: map_trigger_mode(*mode),
                    });
                }
            }
            ActionTriggerConfig::Volume {
                action,
                mode,
                shape,
                transform,
                once,
                ..
            } => {
                let (kind, radius, size) = match shape.kind {
                    VolumeShapeKind::Sphere => (
                        InputVolumeShapeKind::Sphere,
                        shape.radius.unwrap_or(1.0).max(0.0),
                        Vec3::ZERO,
                    ),
                    VolumeShapeKind::Box => {
                        let size_cfg = shape.size.clone().unwrap_or_default();
                        (
                            InputVolumeShapeKind::Box,
                            0.0,
                            Vec3::new(size_cfg.width, size_cfg.height, size_cfg.depth),
                        )
                    }
                };
                resolved_volumes.push(ResolvedVolumeTrigger {
                    action: action.clone(),
                    mode: map_volume_mode(*mode),
                    shape: VolumeShape { kind, radius, size },
                    position: Vec3::new(transform.x, transform.y, transform.z),
                    once: *once,
                    fired: false,
                    inside: false,
                });
            }
        }
    }
    commands.insert_resource(SceneActionTriggers {
        input: resolved_triggers,
        volumes: resolved_volumes,
    });
    commands.insert_resource(ActionStates::default());

    Some(initial_noclip.unwrap_or(false))
}

fn map_trigger_mode(mode: TriggerMode) -> InputTriggerMode {
    match mode {
        TriggerMode::Press => InputTriggerMode::Press,
        TriggerMode::Hold => InputTriggerMode::Hold,
    }
}

fn map_volume_mode(mode: VolumeTriggerMode) -> InputVolumeTriggerMode {
    match mode {
        VolumeTriggerMode::Enter => InputVolumeTriggerMode::Enter,
        VolumeTriggerMode::Exit => InputVolumeTriggerMode::Exit,
        VolumeTriggerMode::Inside => InputVolumeTriggerMode::Inside,
    }
}
