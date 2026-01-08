use bevy::camera::{CameraOutputMode, ClearColorConfig};
use bevy::prelude::*;
use bevy::render::render_resource::BlendState;
use bevy_rapier3d::prelude::{Collider, GravityScale, LockedAxes, RigidBody, Velocity};

use crate::app_config::AppConfig;
use crate::scenes::{
    input::{CameraLookState, PlayerBody, PlayerSpawn, SceneCamera},
    spawn::SceneEntityTag,
    world::WorldConfig,
};

use super::super::render::apply_render_settings;

pub(crate) fn spawn_player_and_cameras(
    world_config: &WorldConfig,
    app_config: &AppConfig,
    initial_noclip: bool,
    commands: &mut Commands,
) {
    let camera_position = Vec3::new(
        world_config.camera.transform.position.x,
        world_config.camera.transform.position.y,
        world_config.camera.transform.position.z,
    );
    let camera_look_at = Vec3::new(
        world_config.camera.transform.look_at.x,
        world_config.camera.transform.look_at.y,
        world_config.camera.transform.look_at.z,
    );
    let camera_up = Vec3::new(
        world_config.camera.transform.up.x,
        world_config.camera.transform.up.y,
        world_config.camera.transform.up.z,
    );
    let basis = Transform::from_translation(camera_position).looking_at(camera_look_at, camera_up);
    let (yaw, pitch, _) = basis.rotation.to_euler(EulerRot::YXZ);
    let pitch = pitch.clamp(-1.4, 1.4);

    commands.insert_resource(PlayerSpawn {
        position: camera_position,
    });
    commands.insert_resource(CameraLookState { pitch });

    let body_half = Vec3::splat(1.0);
    let (body_type, gravity_scale) = if initial_noclip {
        (RigidBody::KinematicPositionBased, 0.0)
    } else {
        (RigidBody::Dynamic, 1.0)
    };
    let body_id = commands
        .spawn((
            Name::new(format!("{}_body", world_config.camera.name)),
            SceneEntityTag,
            PlayerBody,
            body_type,
            Collider::cuboid(body_half.x, body_half.y, body_half.z),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(gravity_scale),
            Velocity::default(),
            Transform::from_translation(camera_position)
                .with_rotation(Quat::from_rotation_y(yaw)),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id();

    let camera_components = (
        Name::new(world_config.camera.name.clone()),
        SceneEntityTag,
        Camera3d::default(),
        SceneCamera,
        Transform::from_translation(Vec3::new(0.0, 0.6, 0.0))
            .with_rotation(Quat::from_rotation_x(pitch)),
    );
    let camera_id = {
        let mut camera = commands.spawn(camera_components);
        if let Some(msaa) = app_config.msaa_component() {
            camera.insert(msaa);
        }
        if let Some(render) = world_config.render.as_ref() {
            apply_render_settings(&mut camera, render);
        }
        camera.id()
    };
    commands.entity(body_id).add_child(camera_id);

    if let Some(msaa) = app_config.msaa_component() {
        commands.spawn((
            SceneEntityTag,
            Camera2d::default(),
            Camera {
                order: 1,
                clear_color: ClearColorConfig::Custom(Color::NONE),
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
                msaa_writeback: false,
                ..default()
            },
            msaa,
        ));
    } else {
        commands.spawn((
            SceneEntityTag,
            Camera2d::default(),
            Camera {
                order: 1,
                clear_color: ClearColorConfig::Custom(Color::NONE),
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
                msaa_writeback: false,
                ..default()
            },
        ));
    }
}
