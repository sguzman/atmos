use bevy::{
    app::AppExit,
    input::keyboard::KeyCode,
    input::mouse::MouseMotion,
    prelude::{
        ButtonInput, MessageReader, MessageWriter, Query, Quat, Res, ResMut, Time, Transform, Vec2,
        Vec3, With, Without,
    },
};
use bevy_rapier3d::prelude::Velocity;

use crate::app_config::AppConfig;

use super::types::{
    CameraControl, CameraLookState, NoclipState, PlayerBody, SceneCamera, SceneInputConfig,
    SceneNoclipConfig, SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState,
};

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 0.6, 0.0);

pub fn apply_camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    app_config: Res<AppConfig>,
    sprint: Option<Res<SprintState>>,
    sprint_config: Option<Res<SceneSprintConfig>>,
    zoom_state: Option<Res<ZoomState>>,
    zoom_config: Option<Res<SceneZoomConfig>>,
    noclip_config: Option<Res<SceneNoclipConfig>>,
    mut noclip_state: Option<ResMut<NoclipState>>,
    look_state: Option<ResMut<CameraLookState>>,
    config: Option<Res<SceneInputConfig>>,
    mut cameras: Query<&mut Transform, (With<SceneCamera>, Without<PlayerBody>)>,
    mut bodies: Query<(&mut Transform, Option<&mut Velocity>), With<PlayerBody>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let Some(config) = config else {
        return;
    };

    if keys.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        mouse_delta += event.delta;
    }

    let Ok(mut camera_transform) = cameras.single_mut() else {
        return;
    };

    let Ok((mut body_transform, body_velocity)) = bodies.single_mut() else {
        apply_free_camera_input(
            &mut camera_transform,
            &config.camera,
            &keys,
            &mouse_delta,
            &app_config,
            sprint.as_ref(),
            sprint_config.as_ref(),
            zoom_state.as_ref(),
            zoom_config.as_ref(),
            time.delta_secs(),
        );
        return;
    };

    let move_cfg = &config.camera.movement;
    let rot_cfg = &config.camera.rotation;
    let dt = time.delta_secs();

    let (yaw_delta, pitch_delta) = resolve_rotation_delta(
        move_cfg.control,
        rot_cfg,
        &keys,
        &mouse_delta,
        &app_config,
        zoom_state.as_ref(),
        zoom_config.as_ref(),
        dt,
    );

    body_transform.rotate_y(yaw_delta);
    if let Some(mut look_state) = look_state {
        let mut pitch = look_state.pitch + pitch_delta;
        pitch = pitch.clamp(-1.4, 1.4);
        look_state.pitch = pitch;
        camera_transform.rotation = Quat::from_rotation_x(pitch);
    } else if pitch_delta != 0.0 {
        camera_transform.rotate_local_x(pitch_delta);
    }

    camera_transform.translation = CAMERA_OFFSET;

    let mut forward_axis = 0.0;
    let mut right_axis = 0.0;
    if let Some(key) = move_cfg.forward {
        if keys.pressed(key) {
            forward_axis += 1.0;
        }
    }
    if let Some(key) = move_cfg.backward {
        if keys.pressed(key) {
            forward_axis -= 1.0;
        }
    }
    if let Some(key) = move_cfg.right {
        if keys.pressed(key) {
            right_axis += 1.0;
        }
    }
    if let Some(key) = move_cfg.left {
        if keys.pressed(key) {
            right_axis -= 1.0;
        }
    }

    if let Some(state) = noclip_state.as_mut() {
        if state.active {
            let mut direction = Vec3::ZERO;
            if forward_axis != 0.0 || right_axis != 0.0 {
                let look_rotation = body_transform.rotation * camera_transform.rotation;
                let forward = look_rotation * -Vec3::Z;
                let right = look_rotation * Vec3::X;
                direction =
                    (forward * forward_axis + right * right_axis).normalize_or_zero();
            }
            let noclip_cfg = noclip_config.as_ref().map(|cfg| &cfg.action);
            let speed = noclip_cfg.map(|cfg| cfg.speed).unwrap_or(move_cfg.speed);
            let acceleration = noclip_cfg.map(|cfg| cfg.acceleration).unwrap_or(10.0);
            let damping = noclip_cfg.map(|cfg| cfg.damping).unwrap_or(5.0);
            let target = direction * speed;

            let accel_factor = (acceleration * dt).min(1.0);
            let damping_factor = (damping * dt).min(1.0);
            if direction.length_squared() > 0.0 {
                state.velocity = state.velocity.lerp(target, accel_factor);
            } else {
                state.velocity = state.velocity.lerp(Vec3::ZERO, damping_factor);
            }

            if let Some(mut velocity) = body_velocity {
                velocity.linvel = state.velocity;
                velocity.angvel = Vec3::ZERO;
            } else {
                body_transform.translation += state.velocity * dt;
            }
            return;
        }
    }

    let mut direction = Vec3::ZERO;
    if forward_axis != 0.0 || right_axis != 0.0 {
        let forward = body_transform.rotation * -Vec3::Z;
        let right = body_transform.rotation * Vec3::X;
        direction = (forward * forward_axis + right * right_axis).normalize_or_zero();
    }

    if let Some(mut velocity) = body_velocity {
        let mut speed = move_cfg.speed;
        if let (Some(state), Some(cfg)) = (sprint.as_ref(), sprint_config.as_ref()) {
            if state.active {
                speed *= cfg.action.multiplier.max(1.0);
            }
        }
        let desired = direction * speed;
        velocity.linvel.x = desired.x;
        velocity.linvel.z = desired.z;
    }
}

fn apply_free_camera_input(
    transform: &mut Transform,
    config: &super::types::ResolvedCameraInputConfig,
    keys: &ButtonInput<KeyCode>,
    mouse_delta: &Vec2,
    app_config: &AppConfig,
    sprint: Option<&Res<SprintState>>,
    sprint_config: Option<&Res<SceneSprintConfig>>,
    zoom_state: Option<&Res<ZoomState>>,
    zoom_config: Option<&Res<SceneZoomConfig>>,
    dt: f32,
) {
    let move_cfg = &config.movement;
    let rot_cfg = &config.rotation;

    let mut forward_axis = 0.0;
    let mut right_axis = 0.0;
    if let Some(key) = move_cfg.forward {
        if keys.pressed(key) {
            forward_axis += 1.0;
        }
    }
    if let Some(key) = move_cfg.backward {
        if keys.pressed(key) {
            forward_axis -= 1.0;
        }
    }
    if let Some(key) = move_cfg.right {
        if keys.pressed(key) {
            right_axis += 1.0;
        }
    }
    if let Some(key) = move_cfg.left {
        if keys.pressed(key) {
            right_axis -= 1.0;
        }
    }

    if forward_axis != 0.0 || right_axis != 0.0 {
        let forward = transform.rotation * -bevy::math::Vec3::Z;
        let right = transform.rotation * bevy::math::Vec3::X;
        let mut direction = forward * forward_axis + right * right_axis;
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            let mut speed = move_cfg.speed;
            if let (Some(state), Some(cfg)) = (sprint, sprint_config) {
                if state.active {
                    speed *= cfg.action.multiplier.max(1.0);
                }
            }
            transform.translation += direction * speed * dt;
        }
    }

    match move_cfg.control {
        CameraControl::Mouse => {
            if mouse_delta.length_squared() > 0.0 {
                let mouse_cfg = &app_config.mouse;
                let mut sensitivity = mouse_cfg.sensitivity;
                if let (Some(state), Some(cfg)) = (zoom_state, zoom_config) {
                    if state.active {
                        sensitivity *= cfg.action.sensitivity_multiplier.max(0.01);
                    }
                }
                let mut yaw = -mouse_delta.x * sensitivity;
                let mut pitch = -mouse_delta.y * sensitivity;
                if mouse_cfg.invert_x {
                    yaw = -yaw;
                }
                if mouse_cfg.invert_y {
                    pitch = -pitch;
                }
                transform.rotate_y(yaw);
                transform.rotate_local_x(pitch);
            }
        }
        CameraControl::Keyboard => {
            let yaw_amount = {
                let mut val = 0.0;
                if let Some(key) = rot_cfg.yaw_left {
                    if keys.pressed(key) {
                        val += 1.0;
                    }
                }
                if let Some(key) = rot_cfg.yaw_right {
                    if keys.pressed(key) {
                        val -= 1.0;
                    }
                }
                val
            };

            let pitch_amount = {
                let mut val = 0.0;
                if let Some(key) = rot_cfg.pitch_up {
                    if keys.pressed(key) {
                        val += 1.0;
                    }
                }
                if let Some(key) = rot_cfg.pitch_down {
                    if keys.pressed(key) {
                        val -= 1.0;
                    }
                }
                val
            };

            let rot_speed = rot_cfg.degrees_per_second.to_radians() * dt;
            if yaw_amount != 0.0 {
                transform.rotate_y(yaw_amount * rot_speed);
            }
            if pitch_amount != 0.0 {
                transform.rotate_local_x(pitch_amount * rot_speed);
            }
        }
    }
}

fn resolve_rotation_delta(
    control: CameraControl,
    rot_cfg: &super::types::ResolvedRotationConfig,
    keys: &ButtonInput<KeyCode>,
    mouse_delta: &Vec2,
    app_config: &AppConfig,
    zoom_state: Option<&Res<ZoomState>>,
    zoom_config: Option<&Res<SceneZoomConfig>>,
    dt: f32,
) -> (f32, f32) {
    match control {
        CameraControl::Mouse => {
            if mouse_delta.length_squared() == 0.0 {
                return (0.0, 0.0);
            }
            let mouse_cfg = &app_config.mouse;
            let mut sensitivity = mouse_cfg.sensitivity;
            if let (Some(state), Some(cfg)) = (zoom_state, zoom_config) {
                if state.active {
                    sensitivity *= cfg.action.sensitivity_multiplier.max(0.01);
                }
            }
            let mut yaw = -mouse_delta.x * sensitivity;
            let mut pitch = -mouse_delta.y * sensitivity;
            if mouse_cfg.invert_x {
                yaw = -yaw;
            }
            if mouse_cfg.invert_y {
                pitch = -pitch;
            }
            (yaw, pitch)
        }
        CameraControl::Keyboard => {
            let yaw_amount = {
                let mut val = 0.0;
                if let Some(key) = rot_cfg.yaw_left {
                    if keys.pressed(key) {
                        val += 1.0;
                    }
                }
                if let Some(key) = rot_cfg.yaw_right {
                    if keys.pressed(key) {
                        val -= 1.0;
                    }
                }
                val
            };

            let pitch_amount = {
                let mut val = 0.0;
                if let Some(key) = rot_cfg.pitch_up {
                    if keys.pressed(key) {
                        val += 1.0;
                    }
                }
                if let Some(key) = rot_cfg.pitch_down {
                    if keys.pressed(key) {
                        val -= 1.0;
                    }
                }
                val
            };

            let rot_speed = rot_cfg.degrees_per_second.to_radians() * dt;
            (yaw_amount * rot_speed, pitch_amount * rot_speed)
        }
    }
}
