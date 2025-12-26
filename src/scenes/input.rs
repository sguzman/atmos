use bevy::{
    app::AppExit,
    input::keyboard::KeyCode,
    input::mouse::{MouseButton, MouseMotion},
    log::warn,
    prelude::{
        ButtonInput, Commands, Component, GlobalTransform, Handle, InheritedVisibility, Mesh,
        Local, Mesh3d, MeshMaterial3d, MessageReader, MessageWriter, Name, Projection, Query, Res,
        ResMut, Resource, StandardMaterial, Time, Transform, Vec2, Vec3, ViewVisibility,
        Visibility, With,
    },
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Ccd, Collider, Friction, Restitution, RigidBody, Velocity,
};

use crate::app_config::AppConfig;
use crate::scenes::bounds::DespawnOutsideBounds;
use crate::scenes::config::{
    ActionBindingConfig, CameraRotationConfig, FovActionConfig, MovementConfig, OverlayInputConfig,
    PhysicsConfig, ShapeConfig, ShootActionConfig, SprintActionConfig, ZoomActionConfig,
};

#[derive(Resource, Debug, Clone)]
pub struct SceneInputConfig {
    pub camera: ResolvedCameraInputConfig,
    pub overlays: Vec<ResolvedOverlayToggle>,
    pub actions: Vec<ResolvedActionBinding>,
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

#[derive(Debug, Clone)]
pub struct ResolvedActionBinding {
    pub name: String,
    pub action: String,
    pub mouse: Option<MouseButton>,
    pub key: Option<KeyCode>,
    pub value: Option<f32>,
}

#[derive(Resource, Clone)]
pub struct SceneShootConfig {
    pub action: ShootActionConfig,
    pub trigger: MouseButton,
    pub name: String,
    pub shape: ShapeConfig,
    pub physics: Option<PhysicsConfig>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource, Clone)]
pub struct SceneSprintConfig {
    pub action: SprintActionConfig,
    pub trigger: KeyCode,
}

#[derive(Resource, Clone)]
pub struct SceneZoomConfig {
    pub action: ZoomActionConfig,
    pub trigger: KeyCode,
}

#[derive(Resource, Default)]
pub struct ZoomState {
    pub active: bool,
    pub base_fov: Option<f32>,
}

#[derive(Clone)]
pub struct FovBinding {
    pub trigger: KeyCode,
    pub fov_degrees: f32,
}

#[derive(Resource, Clone)]
pub struct SceneFovConfig {
    pub action: FovActionConfig,
    pub bindings: Vec<FovBinding>,
}

#[derive(Resource, Default)]
pub struct SprintState {
    pub active: bool,
}

#[derive(Component)]
pub struct SceneCamera;

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

pub fn apply_sprint_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneSprintConfig>>,
    mut state: ResMut<SprintState>,
) {
    let Some(config) = config else {
        return;
    };
    if config.action.toggle && keys.just_pressed(config.trigger) {
        state.active = !state.active;
    }
}

pub fn apply_zoom_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneZoomConfig>>,
    mut state: ResMut<ZoomState>,
    mut cameras: Query<&mut Projection, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

    let Ok(mut projection) = cameras.single_mut() else {
        return;
    };

    if state.base_fov.is_none() {
        if let Projection::Perspective(ref perspective) = *projection {
            state.base_fov = Some(perspective.fov);
        }
    }

    let Some(base_fov) = state.base_fov else {
        return;
    };

    let was_active = state.active;
    if config.action.toggle {
        if keys.just_pressed(config.trigger) {
            state.active = !state.active;
        }
    } else {
        state.active = keys.pressed(config.trigger);
    }

    if !was_active && state.active {
        if let Projection::Perspective(ref perspective) = *projection {
            state.base_fov = Some(perspective.fov);
        }
    }

    if state.active {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = config.action.fov_degrees.to_radians();
        }
    } else if was_active {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = base_fov;
        }
    }
}

pub fn apply_fov_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneFovConfig>>,
    zoom_state: Option<ResMut<ZoomState>>,
    mut cameras: Query<&mut Projection, With<SceneCamera>>,
) {
    let Some(config) = config else {
        return;
    };

    let mut selected = None;
    for binding in &config.bindings {
        if keys.just_pressed(binding.trigger) {
            selected = Some(binding.fov_degrees);
        }
    }

    let Some(fov_degrees) = selected else {
        return;
    };

    let fov_radians = fov_degrees.to_radians();
    if let Some(mut zoom_state) = zoom_state {
        zoom_state.base_fov = Some(fov_radians);
        if zoom_state.active {
            return;
        }
    }

    for mut projection in cameras.iter_mut() {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = fov_radians;
        }
    }
}

#[derive(Default)]
pub struct ShootState {
    accumulator: f32,
    delay_remaining: f32,
}

pub fn apply_shoot_action(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    config: Option<Res<SceneShootConfig>>,
    mut state: Local<ShootState>,
    cameras: Query<&GlobalTransform, With<SceneCamera>>,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };

    if !buttons.pressed(config.trigger) {
        state.accumulator = 0.0;
        state.delay_remaining = 0.0;
        return;
    }

    let Ok(camera) = cameras.single() else {
        return;
    };

    let rate = config.action.rate.max(0.1);
    let interval = 1.0 / rate;
    let forward = camera.forward();
    let spin = Vec3::new(
        config.action.spin.x.to_radians(),
        config.action.spin.y.to_radians(),
        config.action.spin.z.to_radians(),
    );

    let spawn_ball = |commands: &mut Commands| {
        let spawn_pos = camera.translation() + forward * config.action.spawn_offset;
        let mut entity = commands.spawn((
            Name::new(config.name.clone()),
            Mesh3d(config.mesh.clone()),
            MeshMaterial3d(config.material.clone()),
            Transform::from_translation(spawn_pos),
            DespawnOutsideBounds,
            Velocity {
                linvel: forward * config.action.velocity,
                angvel: spin,
            },
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));

        if let Some(physics) = config.physics.as_ref() {
            if !physics.enabled {
                return;
            }
            let rigid_body = resolve_rigid_body(&physics.body_type);
            entity.insert((
                rigid_body,
                Collider::ball(config.shape.radius.unwrap_or(0.2)),
                Restitution::coefficient(physics.restitution),
                Friction::coefficient(physics.friction),
            ));
            if matches!(rigid_body, RigidBody::Dynamic) && physics.mass > 0.0 {
                entity.insert(AdditionalMassProperties::Mass(physics.mass));
            }
            if config.action.ccd {
                entity.insert(Ccd::enabled());
            }
        }
    };

    let dt = time.delta_secs();
    if buttons.just_pressed(config.trigger) {
        state.delay_remaining = config.action.start_delay.max(0.0);
        state.accumulator = 0.0;
        if state.delay_remaining <= 0.0 {
            spawn_ball(&mut commands);
        }
    }

    if state.delay_remaining > 0.0 {
        state.delay_remaining -= dt;
        if state.delay_remaining > 0.0 {
            return;
        }
        spawn_ball(&mut commands);
        state.accumulator = 0.0;
    }

    state.accumulator += dt;
    while state.accumulator >= interval {
        state.accumulator -= interval;
        spawn_ball(&mut commands);
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

pub fn resolve_action_bindings(actions: &[ActionBindingConfig]) -> Vec<ResolvedActionBinding> {
    actions
        .iter()
        .map(|action| ResolvedActionBinding {
            name: action.name.clone(),
            action: action.action.clone(),
            mouse: resolve_mouse_button_or_warn(&action.mouse, "action mouse"),
            key: resolve_key_or_warn(&action.key, "action key"),
            value: action.value,
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

fn resolve_rigid_body(body_type: &str) -> RigidBody {
    match body_type.trim().to_ascii_lowercase().as_str() {
        "fixed" | "static" => RigidBody::Fixed,
        "kinematic_position" | "kinematic_position_based" => {
            RigidBody::KinematicPositionBased
        }
        "kinematic_velocity" | "kinematic_velocity_based" => {
            RigidBody::KinematicVelocityBased
        }
        _ => RigidBody::Dynamic,
    }
}
