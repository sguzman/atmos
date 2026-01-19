use crate::scenes::{
    MeshCacheSettings, TomlAsset,
    bounds::SceneBounds,
    config::ActiveScene,
    entities::EntitiesConfig,
    loaders::{
        ConfigLoad, TomlCache,
        load_entities_config,
        load_world_config,
    },
    spawn::{
        lights::spawn_lights,
        sun::spawn_sun,
        world::spawn_world_entities,
    },
    world::WorldConfig,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    DefaultRapierContext,
    RapierConfiguration,
    RapierContextSimulation,
    TimestepMode,
};

pub(crate) fn load_world_and_entities(
    active_scene: &ActiveScene,
    commands: &mut Commands,
    toml_cache: &mut TomlCache,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
) -> Option<(WorldConfig, EntitiesConfig)>
{
    let world_config: WorldConfig =
        match load_world_config(
            &active_scene.name,
            toml_cache,
            asset_server,
            toml_assets,
        ) {
            ConfigLoad::Pending => {
                return None;
            }
            ConfigLoad::Ready(
                config,
            ) => config,
        };
    let entities_config =
        match load_entities_config(
            &active_scene.name,
            toml_cache,
            asset_server,
            toml_assets,
        ) {
            ConfigLoad::Pending => {
                return None;
            }
            ConfigLoad::Ready(
                config,
            ) => config,
        };

    commands.insert_resource(
        SceneBounds::from(
            world_config.bounds.clone(),
        ),
    );
    Some((
        world_config,
        entities_config,
    ))
}

pub(crate) fn spawn_world_content(
    world_config: &WorldConfig,
    entities_config: &EntitiesConfig,
    active_scene: &ActiveScene,
    mesh_cache: &MeshCacheSettings,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<
        StandardMaterial,
    >,
    asset_server: &AssetServer,
    toml_assets: &Assets<TomlAsset>,
    toml_cache: &mut TomlCache,
    rapier_config: &mut Query<
        &mut RapierConfiguration,
        With<DefaultRapierContext>,
    >,
    rapier_context: &mut Query<
        &mut RapierContextSimulation,
        With<DefaultRapierContext>,
    >,
) -> bool {
    let world_entities_ready =
        spawn_world_entities(
            entities_config,
            commands,
            meshes,
            materials,
            asset_server,
            active_scene,
            mesh_cache,
            toml_assets,
            toml_cache,
        );
    if matches!(
        world_entities_ready,
        ConfigLoad::Pending
    ) {
        return false;
    }

    spawn_sun(
        world_config.sun.as_ref(),
        commands,
        meshes,
        materials,
        asset_server,
        mesh_cache,
    );

    if let Some(skybox) =
        world_config.skybox.clone()
    {
        if let Some(rgb) = crate::scenes::config::parse_color(&skybox.color) {
            commands.insert_resource(ClearColor(Color::srgb_u8(rgb[0], rgb[1], rgb[2])));
        } else {
            warn!(
                "Failed to parse skybox color '{}'; leaving default clear color",
                skybox.color
            );
        }
    }

    if let Ok(mut config) =
        rapier_config.single_mut()
    {
        if let Some(gravity) =
            world_config
                .gravity
                .as_ref()
        {
            config.gravity = Vec3::new(
                gravity.x, gravity.y,
                gravity.z,
            );
        }
    }
    if let Ok(mut context) =
        rapier_context.single_mut()
    {
        if let Some(physics) =
            world_config
                .physics
                .as_ref()
        {
            if let Some(iterations) =
                physics
                    .solver_iterations
            {
                context.integration_parameters.num_solver_iterations =
                    iterations.max(1) as usize;
            }
        }
    }

    let substeps = world_config
        .physics
        .as_ref()
        .and_then(|physics| {
            physics.substeps
        })
        .unwrap_or(1)
        .max(1)
        as usize;
    commands.insert_resource(
        TimestepMode::Variable {
            max_dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps,
        },
    );

    spawn_lights(
        &world_config.lights,
        commands,
    );
    true
}
