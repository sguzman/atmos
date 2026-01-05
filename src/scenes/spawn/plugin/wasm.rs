use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[derive(Component)]
pub(crate) struct WasmKillRoot;

#[derive(Component)]
pub(crate) struct WasmKillButton;

#[derive(Resource, Default)]
pub(crate) struct WasmCursorLockState {
    pub suppress_lock: bool,
}

pub(crate) fn reset_wasm_cursor_state(mut commands: Commands) {
    commands.insert_resource(WasmCursorLockState::default());
}

pub(crate) fn spawn_wasm_kill_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            WasmKillRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                right: Val::Px(16.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            GlobalZIndex(200),
        ))
        .with_children(|parent| {
            parent.spawn((
                Button,
                WasmKillButton,
                BackgroundColor(Color::srgb_u8(30, 30, 30)),
                Node {
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    ..default()
                },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Kill"),
                    TextFont {
                        font: default_font(&asset_server),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

pub(crate) fn cleanup_wasm_kill_button(
    mut commands: Commands,
    roots: Query<Entity, With<WasmKillRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn handle_wasm_kill_button(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (With<WasmKillButton>, Changed<Interaction>),
    >,
    mut cursor_state: ResMut<WasmCursorLockState>,
    mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut cursor) = windows.single_mut() {
                    cursor.grab_mode = CursorGrabMode::None;
                    cursor.visible = true;
                }
                cursor_state.suppress_lock = true;
                *color = BackgroundColor(Color::srgb_u8(50, 50, 50));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb_u8(45, 45, 45));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb_u8(30, 30, 30));
            }
        }
    }
}

pub(crate) fn lock_cursor_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_state: ResMut<WasmCursorLockState>,
    mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if cursor_state.suppress_lock {
        cursor_state.suppress_lock = false;
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if let Ok(mut cursor) = windows.single_mut() {
        if cursor.grab_mode != CursorGrabMode::Locked {
            cursor.grab_mode = CursorGrabMode::Locked;
            cursor.visible = false;
        }
    }
}

fn default_font(asset_server: &AssetServer) -> Handle<Font> {
    let _ = asset_server;
    Handle::<Font>::default()
}
