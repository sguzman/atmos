use bevy::{
    app::AppExit,
    input::keyboard::KeyCode,
    input::mouse::MouseMotion,
    log::warn,
    prelude::{
        ButtonInput, Component, MessageReader, MessageWriter, Query, Res, Resource, Time,
        Transform, Vec2, With,
    },
};

use crate::app_config::AppConfig;
use crate::scenes::config::{CameraRotationConfig, MovementConfig, OverlayInputConfig};

#[derive(Resource, Debug, Clone)]
pub struct SceneInputConfig {
    pub camera: ResolvedCameraInputConfig,
    pub overlays: Vec<ResolvedOverlayToggle>,
}

#[derive(Debug, Clone)]
pub struct ResolvedCameraInputConfig {
    pub movement: ResolvedMovementConfig,
    pub rotation: ResolvedRotationConfig,
}

#[derive(Debug, Clone)]
pub struct ResolvedMovementConfig {
    pub control: CameraControl,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraControl {
    Mouse,
    Keyboard,
}

#[derive(Debug, Clone)]
pub struct ResolvedOverlayToggle {
    pub name: String,
    pub toggle: Option<KeyCode>,
}

#[derive(Component)]
pub struct SceneCamera;

pub fn apply_camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    app_config: Res<AppConfig>,
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
                transform.translation += direction * move_cfg.speed * dt;
            }
        }

        match move_cfg.control {
            CameraControl::Mouse => {
                if mouse_delta.length_squared() > 0.0 {
                    let mouse_cfg = &app_config.mouse;
                    let mut yaw = -mouse_delta.x * mouse_cfg.sensitivity;
                    let mut pitch = -mouse_delta.y * mouse_cfg.sensitivity;
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

pub fn resolve_camera_input_config(
    movement: &MovementConfig,
    rotation: &CameraRotationConfig,
) -> ResolvedCameraInputConfig {
    ResolvedCameraInputConfig {
        movement: ResolvedMovementConfig {
            control: resolve_control_or_warn(&movement.control, "camera control"),
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

pub fn resolve_overlay_toggles(overlays: &[OverlayInputConfig]) -> Vec<ResolvedOverlayToggle> {
    overlays
        .iter()
        .map(|ovr| ResolvedOverlayToggle {
            name: ovr.name.clone(),
            toggle: resolve_key_or_warn(&ovr.toggle, &format!("overlay toggle '{}'", ovr.name)),
        })
        .collect()
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

fn resolve_control_or_warn(control: &str, action: &str) -> CameraControl {
    let normalized = control.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "mouse" => CameraControl::Mouse,
        "keyboard" => CameraControl::Keyboard,
        "" => CameraControl::Mouse,
        _ => {
            warn!("Unrecognized control '{control}' for {action}; defaulting to mouse.");
            CameraControl::Mouse
        }
    }
}

fn resolve_key(key: &str) -> Option<KeyCode> {
    let normalized = key.trim().to_ascii_lowercase();
    // Letters
    if normalized.len() == 1 {
        let c = normalized.as_bytes()[0] as char;
        if ('a'..='z').contains(&c) {
            return Some(match c {
                'a' => KeyCode::KeyA,
                'b' => KeyCode::KeyB,
                'c' => KeyCode::KeyC,
                'd' => KeyCode::KeyD,
                'e' => KeyCode::KeyE,
                'f' => KeyCode::KeyF,
                'g' => KeyCode::KeyG,
                'h' => KeyCode::KeyH,
                'i' => KeyCode::KeyI,
                'j' => KeyCode::KeyJ,
                'k' => KeyCode::KeyK,
                'l' => KeyCode::KeyL,
                'm' => KeyCode::KeyM,
                'n' => KeyCode::KeyN,
                'o' => KeyCode::KeyO,
                'p' => KeyCode::KeyP,
                'q' => KeyCode::KeyQ,
                'r' => KeyCode::KeyR,
                's' => KeyCode::KeyS,
                't' => KeyCode::KeyT,
                'u' => KeyCode::KeyU,
                'v' => KeyCode::KeyV,
                'w' => KeyCode::KeyW,
                'x' => KeyCode::KeyX,
                'y' => KeyCode::KeyY,
                'z' => KeyCode::KeyZ,
                _ => unreachable!(),
            });
        }
    }

    match normalized.as_str() {
        // Arrow keys
        "left" | "arrowleft" => Some(KeyCode::ArrowLeft),
        "right" | "arrowright" => Some(KeyCode::ArrowRight),
        "up" | "arrowup" => Some(KeyCode::ArrowUp),
        "down" | "arrowdown" => Some(KeyCode::ArrowDown),

        // Digits
        "0" => Some(KeyCode::Digit0),
        "1" => Some(KeyCode::Digit1),
        "2" => Some(KeyCode::Digit2),
        "3" => Some(KeyCode::Digit3),
        "4" => Some(KeyCode::Digit4),
        "5" => Some(KeyCode::Digit5),
        "6" => Some(KeyCode::Digit6),
        "7" => Some(KeyCode::Digit7),
        "8" => Some(KeyCode::Digit8),
        "9" => Some(KeyCode::Digit9),

        // Punctuation by name
        "space" => Some(KeyCode::Space),
        "enter" | "return" => Some(KeyCode::Enter),
        "escape" | "esc" => Some(KeyCode::Escape),
        "tab" => Some(KeyCode::Tab),
        "backspace" => Some(KeyCode::Backspace),
        "minus" | "dash" | "hyphen" => Some(KeyCode::Minus),
        "equal" | "equals" | "plus" => Some(KeyCode::Equal),
        "lbracket" | "leftbracket" => Some(KeyCode::BracketLeft),
        "rbracket" | "rightbracket" => Some(KeyCode::BracketRight),
        "backslash" => Some(KeyCode::Backslash),
        "semicolon" => Some(KeyCode::Semicolon),
        "quote" | "apostrophe" => Some(KeyCode::Quote),
        "comma" => Some(KeyCode::Comma),
        "period" | "dot" => Some(KeyCode::Period),
        "slash" | "forwardslash" => Some(KeyCode::Slash),
        "grave" | "backtick" => Some(KeyCode::Backquote),

        // Modifiers and common specials
        "shift" | "lshift" => Some(KeyCode::ShiftLeft),
        "rshift" => Some(KeyCode::ShiftRight),
        "ctrl" | "control" | "lctrl" => Some(KeyCode::ControlLeft),
        "rctrl" => Some(KeyCode::ControlRight),
        "alt" | "lalt" => Some(KeyCode::AltLeft),
        "ralt" => Some(KeyCode::AltRight),
        "meta" | "lmeta" | "super" | "lsuper" | "win" | "lwin" => Some(KeyCode::SuperLeft),
        "rmeta" | "rsuper" | "rwin" => Some(KeyCode::SuperRight),

        _ => None,
    }
}
