use bevy::prelude::*;

use crate::scenes::{
    config::{ActiveScene, InputConfig, ShapeKind},
    input::{
        FovBinding, GrabHover, GrabState, NoclipState, SceneFovConfig, SceneGrabConfig,
        SceneGrenadeConfig, SceneJumpConfig, SceneNoclipConfig, SceneReloadConfig, SceneShootConfig,
        SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState,
    },
    loaders::{
        load_entity_template_from_path, load_grab_action_config, load_grenade_action_config,
        load_jump_action_config, load_noclip_action_config, load_reload_action_config,
        load_shoot_action_config, load_sprint_action_config, load_zoom_action_config, ConfigLoad,
        TomlCache,
    },
    MeshCacheSettings, TomlAsset,
};

pub(crate) fn setup_actions(
    input_config: &InputConfig,
    active_scene: &ActiveScene,
    mesh_cache: &MeshCacheSettings,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
    toml_cache: &mut TomlCache,
) -> Option<bool> {
    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("shoot-balls.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_mouse_button_or_warn(&action_binding.mouse, "shoot")
        {
            let action = match load_shoot_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
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
                    return None;
                };

                let Some(shape) = projectile.shape.clone() else {
                    warn!("Projectile template has no shape; shoot action disabled.");
                    return None;
                };
                if shape.kind != ShapeKind::Sphere {
                    warn!("Projectile template is not a sphere; shoot action disabled.");
                    return None;
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
                    action,
                    trigger,
                    name: projectile.name.clone(),
                    shape,
                    physics: projectile.physics.clone(),
                    mesh: sphere_mesh,
                    material: sphere_material,
                });
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("grenade.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "grenade")
        {
            let action = match load_grenade_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
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
                    return None;
                };

                let Some(mut shape) = projectile.shape.clone() else {
                    warn!("Grenade template has no shape; grenade action disabled.");
                    return None;
                };
                if shape.kind != ShapeKind::Sphere {
                    warn!("Grenade template is not a sphere; grenade action disabled.");
                    return None;
                }
                if action.radius > 0.0 {
                    shape.radius = Some(action.radius);
                }
                if !action.color.trim().is_empty() {
                    shape.color = Some(action.color.clone());
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
                    action,
                    trigger,
                    name: projectile.name.clone(),
                    shape,
                    physics: projectile.physics.clone(),
                    mesh: sphere_mesh,
                    material: sphere_material,
                });
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("sprint.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "sprint")
        {
            let action = match load_sprint_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
                commands.insert_resource(SceneSprintConfig { action, trigger });
                commands.insert_resource(SprintState::default());
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("zoom.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "zoom")
        {
            let action = match load_zoom_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
                commands.insert_resource(SceneZoomConfig { action, trigger });
                commands.insert_resource(ZoomState::default());
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("jump.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "jump")
        {
            let action = match load_jump_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
                commands.insert_resource(SceneJumpConfig { action, trigger });
            }
        }
    }

    let mut initial_noclip = None;
    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("noclip.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "noclip")
        {
            let action = match load_noclip_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
                let state = NoclipState {
                    active: action.enabled,
                    velocity: Vec3::ZERO,
                    fast: false,
                };
                let speed_toggle_key = crate::scenes::input::resolve_key_or_warn(
                    &action.speed_toggle_key,
                    "noclip speed toggle",
                );
                let up_key =
                    crate::scenes::input::resolve_key_or_warn(&action.up_key, "noclip up");
                let down_key =
                    crate::scenes::input::resolve_key_or_warn(&action.down_key, "noclip down");
                initial_noclip = Some(action.enabled);
                commands.insert_resource(SceneNoclipConfig {
                    action,
                    trigger,
                    speed_toggle_key,
                    up_key,
                    down_key,
                });
                commands.insert_resource(state);
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("grab.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "grab")
        {
            let action = match load_grab_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if let Some(action) = action {
                let rgb = crate::scenes::config::parse_color(&action.outline.color)
                    .unwrap_or([0, 255, 255]);
                let outline_color = Color::srgb_u8(rgb[0], rgb[1], rgb[2]);
                commands.insert_resource(SceneGrabConfig {
                    action,
                    trigger,
                    outline_color,
                });
                commands.insert_resource(GrabState::default());
                commands.insert_resource(GrabHover::default());
            }
        }
    }

    if let Some(action_binding) = input_config
        .actions
        .iter()
        .find(|action| action.action.ends_with("reload.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "reload")
        {
            let action = match load_reload_action_config(
                &active_scene.name,
                &action_binding.action,
                toml_cache,
                asset_server,
                toml_assets,
            ) {
                ConfigLoad::Pending => return None,
                ConfigLoad::Ready(action) => action,
            };
            if action.is_some() {
                commands.insert_resource(SceneReloadConfig { trigger });
            }
        }
    }

    let mut fov_bindings = Vec::new();
    for action_binding in input_config
        .actions
        .iter()
        .filter(|action| action.action.ends_with("fov.toml"))
    {
        if let Some(trigger) =
            crate::scenes::input::resolve_key_or_warn(&action_binding.key, "fov")
        {
            if let Some(fov_value) = action_binding.value {
                fov_bindings.push(FovBinding {
                    trigger,
                    fov_degrees: fov_value,
                });
            } else {
                warn!(
                    "Fov action '{}' is missing a value; binding skipped.",
                    action_binding.name
                );
            }
        }
    }

    if !fov_bindings.is_empty() {
        commands.insert_resource(SceneFovConfig {
            bindings: fov_bindings,
        });
    }

    Some(initial_noclip.unwrap_or(false))
}
