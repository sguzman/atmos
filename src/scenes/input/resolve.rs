use bevy::{
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    log::warn,
};

use crate::scenes::config::{CameraRotationConfig, MovementConfig, OverlayInputConfig};

use super::types::{
    CameraControl, ResolvedCameraInputConfig, ResolvedMovementConfig, ResolvedOverlayToggle,
    ResolvedRotationConfig,
};

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

pub fn resolve_key_or_warn(key: &str, action: &str) -> Option<KeyCode> {
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

pub fn resolve_mouse_button_or_warn(button: &str, action: &str) -> Option<MouseButton> {
    if button.trim().is_empty() {
        return None;
    }
    match resolve_mouse_button(button) {
        Some(button) => Some(button),
        None => {
            warn!("Unrecognized mouse button '{button}' for {action}; binding disabled.");
            None
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

fn resolve_mouse_button(button: &str) -> Option<MouseButton> {
    match button.trim().to_ascii_lowercase().as_str() {
        "left" | "lmb" => Some(MouseButton::Left),
        "right" | "rmb" => Some(MouseButton::Right),
        "middle" | "mmb" => Some(MouseButton::Middle),
        _ => None,
    }
}
