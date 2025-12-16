use bevy::{
    log::{info, warn},
    prelude::*,
};

use crate::scenes::{
    config::{
        ActiveScene, CameraConfig, CircleConfig, CubeConfig, InputConfig, LightConfig,
        PillarComboConfig, RectangleConfig,
    },
    input::{
        apply_camera_input, resolve_camera_input_config, resolve_overlay_toggles, SceneCamera,
        SceneInputConfig,
    },
    loaders::{
        load_camera_config, load_circle_config, load_cube_config, load_input_config,
        load_light_config, load_pillar_combo_config, load_rectangle_config, load_skybox_config,
        load_sun_config, load_top_light_config, load_world_config,
    },
    world::WorldConfig,
};

use super::lights::spawn_lights;
use super::logging::{log_camera, log_lights};
use super::overlay::{spawn_overlays_from_config, OverlayTag};
use super::sun::spawn_sun;
use super::world::spawn_world_entities;

pub struct ScenePlugin {
    scene: &'static str,
}

impl ScenePlugin {
    pub const fn new(scene: &'static str) -> Self {
        Self { scene }
    }
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveScene {
            name: self.scene.to_string(),
        });
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, apply_camera_input);
        app.add_systems(
            PostStartup,
            (log_lights, log_camera, spawn_overlays_from_config).after(setup_scene),
        );
        app.add_systems(Update, toggle_overlays);
    }
}

fn setup_scene(
    active_scene: Res<ActiveScene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!(
        "Bootstrapping scene '{}' with inspector overlay enabled.",
        active_scene.name
    );

    let input_config: InputConfig = load_input_config(&active_scene.name);
    let camera_input =
        resolve_camera_input_config(&input_config.camera.movement, &input_config.camera.rotation);
    commands.insert_resource(SceneInputConfig {
        camera: camera_input,
        overlays: resolve_overlay_toggles(&input_config.overlays),
    });

    let cube_config: CubeConfig = load_cube_config(&active_scene.name);
    let camera_config: CameraConfig = load_camera_config(&active_scene.name);
    let circle_config: CircleConfig = load_circle_config(&active_scene.name);
    let rectangle_config: RectangleConfig = load_rectangle_config(&active_scene.name);
    let top_light_template: LightConfig = load_top_light_config(&active_scene.name);
    let combo_config: PillarComboConfig = load_pillar_combo_config(&active_scene.name);
    let world_config: WorldConfig = load_world_config(&active_scene.name);
    let light_config = load_light_config(&active_scene.name);
    let sun_config = load_sun_config(&active_scene.name);
    let skybox_config = load_skybox_config(&active_scene.name);

    if cube_config.physics.enabled {
        warn!(
            "Cube physics config is enabled for scene '{}' but not applied yet (body_type={}, mass={}, restitution={}, friction={}).",
            active_scene.name,
            cube_config.physics.body_type,
            cube_config.physics.mass,
            cube_config.physics.restitution,
            cube_config.physics.friction,
        );
    }

    spawn_world_entities(
        &world_config,
        &circle_config,
        &cube_config,
        &rectangle_config,
        &top_light_template,
        &combo_config,
        &mut commands,
        &mut meshes,
        &mut materials,
        &active_scene,
    );

    // sun derived from world config
    spawn_sun(sun_config.as_ref(), &mut commands, &mut meshes, &mut materials, &active_scene);

    // skybox clear color
    if let Some(skybox) = skybox_config {
        if let Some(rgb) = crate::scenes::config::parse_color(&skybox.color) {
            commands.insert_resource(ClearColor(Color::srgb_u8(rgb[0], rgb[1], rgb[2])));
        } else {
            warn!("Failed to parse skybox color '{}'; leaving default clear color", skybox.color);
        }
    }

    // lights
    spawn_lights(&light_config, &mut commands);

    // camera
    commands.spawn((
        Name::new(camera_config.name),
        Camera3d::default(),
        SceneCamera,
        Transform::from_xyz(
            camera_config.transform.position.x,
            camera_config.transform.position.y,
            camera_config.transform.position.z,
        )
        .looking_at(
            Vec3::new(
                camera_config.transform.look_at.x,
                camera_config.transform.look_at.y,
                camera_config.transform.look_at.z,
            ),
            Vec3::new(
                camera_config.transform.up.x,
                camera_config.transform.up.y,
                camera_config.transform.up.z,
            ),
        ),
    ));

    // UI overlay camera
    commands.spawn((Camera2d::default(), Camera { order: 1, ..default() }));
}

fn toggle_overlays(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneInputConfig>>,
    mut overlays: Query<(&OverlayTag, &mut Visibility)>,
) {
    let Some(config) = config else { return };
    for overlay in &config.overlays {
        let Some(key) = overlay.toggle else { continue };
        if keys.just_pressed(key) {
            for (_tag, mut vis) in overlays
                .iter_mut()
                .filter(|(tag, _)| tag.name == overlay.name)
            {
                vis.toggle_visible_hidden();
            }
        }
    }
}
