use bevy::{
    app::AppExit,
    input::keyboard::KeyCode,
    input::mouse::MouseMotion,
    prelude::{
        ButtonInput, MessageReader, MessageWriter, Query, Res, Time, Transform, Vec2, With,
    },
};

use crate::app_config::AppConfig;

use super::types::{CameraControl, SceneCamera, SceneInputConfig, SceneSprintConfig, SceneZoomConfig, SprintState, ZoomState};

pub fn apply_camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    app_config: Res<AppConfig>,
    sprint: Option<Res<SprintState>>,
    sprint_config: Option<Res<SceneSprintConfig>>,
    zoom_state: Option<Res<ZoomState>>,
    zoom_config: Option<Res<SceneZoomConfig>>,
    config: Option<Res<SceneInputConfig>>,
    mut cameras: Query<&mut Transform, With<SceneCamera>>,
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

    for mut transform in cameras.iter_mut() {
        let move_cfg = &config.camera.movement;
        let rot_cfg = &config.camera.rotation;
        let dt = time.delta_secs();

        // Movement
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
                if let (Some(state), Some(cfg)) = (sprint.as_ref(), sprint_config.as_ref()) {
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
                    if let (Some(state), Some(cfg)) = (zoom_state.as_ref(), zoom_config.as_ref()) {
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
}
