use bevy::{
    input::keyboard::KeyCode,
    log::warn,
    prelude::{ButtonInput, Component, Query, Res, Resource, Time, Transform, With},
};

use crate::scenes::config::{CameraRotationConfig, MovementConfig};

#[derive(Resource, Debug, Clone)]
pub struct SceneInputConfig {
    pub camera: ResolvedCameraInputConfig,
}

#[derive(Debug, Clone)]
pub struct ResolvedCameraInputConfig {
    pub movement: ResolvedMovementConfig,
    pub rotation: ResolvedRotationConfig,
}

#[derive(Debug, Clone)]
pub struct ResolvedMovementConfig {
    pub speed: f32,
    pub forward: Option<KeyCode>,
    pub backward: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
}

#[derive(Debug, Clone)]
pub struct ResolvedRotationConfig {
    pub degrees_per_second: f32,
    pub yaw_left: Option<KeyCode>,
    pub yaw_right: Option<KeyCode>,
    pub pitch_up: Option<KeyCode>,
    pub pitch_down: Option<KeyCode>,
}

#[derive(Component)]
pub struct SceneCamera;

pub fn apply_camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneInputConfig>>,
    mut cameras: Query<&mut Transform, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

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
                transform.translation += direction * move_cfg.speed * dt;
            }
        }

        // Rotation
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

pub fn resolve_camera_input_config(
    movement: &MovementConfig,
    rotation: &CameraRotationConfig,
) -> ResolvedCameraInputConfig {
    ResolvedCameraInputConfig {
        movement: ResolvedMovementConfig {
            speed: movement.speed,
            forward: resolve_key_or_warn(&movement.forward, "camera forward"),
            backward: resolve_key_or_warn(&movement.backward, "camera backward"),
            left: resolve_key_or_warn(&movement.left, "camera left"),
            right: resolve_key_or_warn(&movement.right, "camera right"),
        },
        rotation: ResolvedRotationConfig {
            degrees_per_second: rotation.degrees_per_second,
            yaw_left: resolve_key_or_warn(&rotation.yaw_left, "camera yaw left"),
            yaw_right: resolve_key_or_warn(&rotation.yaw_right, "camera yaw right"),
            pitch_up: resolve_key_or_warn(&rotation.pitch_up, "camera pitch up"),
            pitch_down: resolve_key_or_warn(&rotation.pitch_down, "camera pitch down"),
        },
    }
}

fn resolve_key_or_warn(key: &str, action: &str) -> Option<KeyCode> {
    if key.trim().is_empty() {
        return None;
    }
    match resolve_key(key) {
        Some(code) => Some(code),
        None => {
            warn!("Unrecognized key '{key}' for {action}; binding disabled.");
            None
        }
    }
}

fn resolve_key(key: &str) -> Option<KeyCode> {
    let normalized = key.trim().to_ascii_uppercase();
    match normalized.as_str() {
        "W" => Some(KeyCode::KeyW),
        "A" => Some(KeyCode::KeyA),
        "S" => Some(KeyCode::KeyS),
        "D" => Some(KeyCode::KeyD),
        "ARROWLEFT" | "LEFT" => Some(KeyCode::ArrowLeft),
        "ARROWRIGHT" | "RIGHT" => Some(KeyCode::ArrowRight),
        "ARROWUP" | "UP" => Some(KeyCode::ArrowUp),
        "ARROWDOWN" | "DOWN" => Some(KeyCode::ArrowDown),
        _ => None,
    }
}
