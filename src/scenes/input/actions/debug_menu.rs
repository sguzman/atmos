use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::time::{Time, Virtual};
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy::post_process::bloom::Bloom;
use bevy::pbr::DistanceFog;
use bevy_rapier3d::prelude::{DefaultRapierContext, RapierConfiguration, TimestepMode};

use crate::app_config::{AppConfig, AppMode};
use crate::scenes::input::{ActionStates, DebugMenuPage, DebugMenuState, SceneCamera, ScenePauseConfig, ZoomState};
use crate::scenes::spawn::SunLight;

#[derive(Component)]
pub struct DebugMenuUiTag;

#[derive(Component, Clone)]
pub(crate) struct DebugMenuButton {
    action: DebugMenuAction,
}

#[derive(Clone)]
enum DebugMenuAction {
    Noop,
    Open(DebugMenuPage),
    Back,
    ToggleBloom,
    ToggleFog,
    ToggleDlss,
    ToggleRayTracing,
    AdjustFov(f32),
    AdjustGravity(f32),
    TogglePhysics,
    AdjustSunBrightness(f32),
    ToggleSunShadows,
}

struct DebugMenuEntry {
    label: String,
    action: DebugMenuAction,
}

pub fn apply_debug_menu_toggle(
    app_config: Res<AppConfig>,
    config: Option<Res<ScenePauseConfig>>,
    states: Option<Res<ActionStates>>,
    debug_state: Option<ResMut<DebugMenuState>>,
    mut commands: Commands,
    ui_nodes: Query<Entity, With<DebugMenuUiTag>>,
    mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>,
    time: Option<ResMut<Time<Virtual>>>,
    timestep: Option<ResMut<TimestepMode>>,
    cameras: Query<(Entity, &Projection, Option<&Bloom>, Option<&DistanceFog>), With<SceneCamera>>,
    rapier_config: Query<&RapierConfiguration, With<DefaultRapierContext>>,
    sun: Query<&DirectionalLight, With<SunLight>>,
) {
    if !debug_menu_enabled(&app_config) {
        return;
    }
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut debug_state) = debug_state else {
        return;
    };

    if !states.get(&config.id).just_pressed {
        return;
    }

    if debug_state.active {
        close_debug_menu(
            &app_config,
            &mut debug_state,
            &mut commands,
            &ui_nodes,
            &mut windows,
            time,
            timestep,
        );
        return;
    }

    open_debug_menu(
        &app_config,
        &mut debug_state,
        &mut windows,
        time,
        timestep,
        &cameras,
        &rapier_config,
        &sun,
    );
}

pub fn update_debug_menu_ui(
    app_config: Res<AppConfig>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    debug_state: Option<ResMut<DebugMenuState>>,
    mut commands: Commands,
    ui_nodes: Query<Entity, With<DebugMenuUiTag>>,
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &Children, &DebugMenuButton), Changed<Interaction>>,
    mut text_colors: Query<&mut TextColor>,
    mut projections: Query<&mut Projection, With<SceneCamera>>,
    mut zoom_state: Option<ResMut<ZoomState>>,
    camera_entities: Query<Entity, With<SceneCamera>>,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    mut sun: Query<&mut DirectionalLight, With<SunLight>>,
    asset_server: Res<AssetServer>,
) {
    if !debug_menu_enabled(&app_config) {
        return;
    }
    let Some(mut debug_state) = debug_state else {
        return;
    };
    if !debug_state.active {
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Right) {
        if debug_state.stack.len() > 1 {
            debug_state.stack.pop();
            debug_state.needs_refresh = true;
        }
    }

    for (interaction, mut background, children, button) in buttons.iter_mut() {
        let (bg_color, text_color) = match *interaction {
            Interaction::Pressed => (Color::srgba(0.35, 0.35, 0.35, 0.95), Color::BLACK),
            Interaction::Hovered => (Color::srgba(0.25, 0.25, 0.25, 0.9), Color::WHITE),
            Interaction::None => (Color::srgba(0.1, 0.1, 0.1, 0.85), Color::WHITE),
        };
        *background = bg_color.into();
        for &child in children {
            if let Ok(mut color) = text_colors.get_mut(child) {
                color.0 = text_color;
            }
        }

        if *interaction == Interaction::Pressed {
            apply_debug_menu_action(
                &button.action,
                &mut debug_state,
                &mut commands,
                &mut projections,
                &mut zoom_state,
                &camera_entities,
                &mut rapier_config,
                &mut sun,
            );
        }
    }

    if debug_state.needs_refresh {
        for entity in &ui_nodes {
            safe_despawn(&mut commands, entity);
        }
        spawn_debug_menu_ui(
            &mut commands,
            &asset_server,
            &debug_state,
        );
        debug_state.needs_refresh = false;
    }
}

fn debug_menu_enabled(app_config: &AppConfig) -> bool {
    matches!(app_config.mode, AppMode::Dev) && app_config.debug_menu.enabled
}

fn open_debug_menu(
    app_config: &AppConfig,
    debug_state: &mut DebugMenuState,
    windows: &mut Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut time: Option<ResMut<Time<Virtual>>>,
    mut timestep: Option<ResMut<TimestepMode>>,
    cameras: &Query<(Entity, &Projection, Option<&Bloom>, Option<&DistanceFog>), With<SceneCamera>>,
    rapier_config: &Query<&RapierConfiguration, With<DefaultRapierContext>>,
    sun: &Query<&DirectionalLight, With<SunLight>>,
) {
    debug_state.active = true;
    if debug_state.stack.is_empty() {
        debug_state.stack.push(DebugMenuPage::Root);
    }
    debug_state.needs_refresh = true;

    initialize_debug_menu_settings(debug_state, cameras, rapier_config, sun);

    if let Ok(mut cursor) = windows.single_mut() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }

    if app_config.debug_menu.pause_scene {
        if let Some(mut time) = time.take() {
            time.pause();
        }
        if let Some(mut timestep) = timestep.take() {
            if let TimestepMode::Variable { time_scale, .. } = &mut *timestep {
                debug_state.stored_time_scale = *time_scale;
                *time_scale = 0.0;
            }
        }
    }

}

fn close_debug_menu(
    app_config: &AppConfig,
    debug_state: &mut DebugMenuState,
    commands: &mut Commands,
    ui_nodes: &Query<Entity, With<DebugMenuUiTag>>,
    windows: &mut Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut time: Option<ResMut<Time<Virtual>>>,
    mut timestep: Option<ResMut<TimestepMode>>,
) {
    debug_state.active = false;
    debug_state.needs_refresh = false;

    for entity in ui_nodes {
        safe_despawn(commands, entity);
    }

    if let Ok(mut cursor) = windows.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    if app_config.debug_menu.pause_scene {
        if let Some(mut time) = time.take() {
            time.unpause();
        }
        if let Some(mut timestep) = timestep.take() {
            if let TimestepMode::Variable { time_scale, .. } = &mut *timestep {
                let restore = if debug_state.stored_time_scale > 0.0 {
                    debug_state.stored_time_scale
                } else {
                    1.0
                };
                *time_scale = restore;
            }
        }
    }
}

fn initialize_debug_menu_settings(
    debug_state: &mut DebugMenuState,
    cameras: &Query<(Entity, &Projection, Option<&Bloom>, Option<&DistanceFog>), With<SceneCamera>>,
    rapier_config: &Query<&RapierConfiguration, With<DefaultRapierContext>>,
    sun: &Query<&DirectionalLight, With<SunLight>>,
) {
    if debug_state.settings.initialized {
        return;
    }

    if let Ok((_entity, projection, bloom, fog)) = cameras.single() {
        if let Projection::Perspective(perspective) = projection {
            debug_state.settings.fov_degrees = perspective.fov.to_degrees();
        }
        debug_state.settings.bloom_enabled = bloom.is_some();
        debug_state.settings.bloom = bloom.cloned();
        debug_state.settings.fog_enabled = fog.is_some();
        debug_state.settings.fog = fog.cloned();
    }

    if let Ok(config) = rapier_config.single() {
        debug_state.settings.gravity = Vec3::new(config.gravity.x, config.gravity.y, config.gravity.z);
        debug_state.settings.physics_enabled = config.physics_pipeline_active;
    }

    if let Ok(light) = sun.single() {
        debug_state.settings.sun_present = true;
        debug_state.settings.sun_brightness = light.illuminance;
        debug_state.settings.sun_shadows = light.shadows_enabled;
    }

    debug_state.settings.initialized = true;
}

fn apply_debug_menu_action(
    action: &DebugMenuAction,
    debug_state: &mut DebugMenuState,
    commands: &mut Commands,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
    camera_entities: &Query<Entity, With<SceneCamera>>,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    match action {
        DebugMenuAction::Noop => {}
        DebugMenuAction::Open(page) => {
            debug_state.stack.push(*page);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::Back => {
            if debug_state.stack.len() > 1 {
                debug_state.stack.pop();
                debug_state.needs_refresh = true;
            }
        }
        DebugMenuAction::ToggleBloom => {
            debug_state.settings.bloom_enabled = !debug_state.settings.bloom_enabled;
            apply_bloom_toggle(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleFog => {
            debug_state.settings.fog_enabled = !debug_state.settings.fog_enabled;
            apply_fog_toggle(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleDlss => {
            debug_state.settings.dlss_enabled = !debug_state.settings.dlss_enabled;
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleRayTracing => {
            debug_state.settings.ray_tracing_enabled = !debug_state.settings.ray_tracing_enabled;
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::AdjustFov(delta) => {
            let next = (debug_state.settings.fov_degrees + delta).clamp(30.0, 160.0);
            debug_state.settings.fov_degrees = next;
            apply_fov(debug_state, projections, zoom_state);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::AdjustGravity(delta) => {
            debug_state.settings.gravity.y += delta;
            apply_gravity(debug_state, rapier_config);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::TogglePhysics => {
            debug_state.settings.physics_enabled = !debug_state.settings.physics_enabled;
            apply_physics_toggle(debug_state, rapier_config);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::AdjustSunBrightness(delta) => {
            if debug_state.settings.sun_present {
                debug_state.settings.sun_brightness = (debug_state.settings.sun_brightness + delta).max(0.0);
                apply_sun(debug_state, sun);
                debug_state.needs_refresh = true;
            }
        }
        DebugMenuAction::ToggleSunShadows => {
            if debug_state.settings.sun_present {
                debug_state.settings.sun_shadows = !debug_state.settings.sun_shadows;
                apply_sun(debug_state, sun);
                debug_state.needs_refresh = true;
            }
        }
    }
}

fn apply_fov(
    debug_state: &DebugMenuState,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
) {
    let fov_radians = debug_state.settings.fov_degrees.to_radians();
    if let Some(zoom_state) = zoom_state.as_mut() {
        zoom_state.base_fov = Some(fov_radians);
        if zoom_state.active {
            return;
        }
    }
    for mut projection in projections.iter_mut() {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = fov_radians;
        }
    }
}

fn apply_bloom_toggle(
    debug_state: &DebugMenuState,
    commands: &mut Commands,
    cameras: &Query<Entity, With<SceneCamera>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    if debug_state.settings.bloom_enabled {
        let bloom = debug_state
            .settings
            .bloom
            .clone()
            .unwrap_or_else(Bloom::default);
        commands.entity(camera).insert(bloom);
    } else {
        commands.entity(camera).remove::<Bloom>();
    }
}

fn apply_fog_toggle(
    debug_state: &DebugMenuState,
    commands: &mut Commands,
    cameras: &Query<Entity, With<SceneCamera>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    if debug_state.settings.fog_enabled {
        let fog = debug_state
            .settings
            .fog
            .clone()
            .unwrap_or_else(DistanceFog::default);
        commands.entity(camera).insert(fog);
    } else {
        commands.entity(camera).remove::<DistanceFog>();
    }
}

fn apply_gravity(
    debug_state: &DebugMenuState,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = debug_state.settings.gravity;
    }
}

fn apply_physics_toggle(
    debug_state: &DebugMenuState,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if let Ok(mut config) = rapier_config.single_mut() {
        config.physics_pipeline_active = debug_state.settings.physics_enabled;
    }
}

fn apply_sun(
    debug_state: &DebugMenuState,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    if let Ok(mut light) = sun.single_mut() {
        light.illuminance = debug_state.settings.sun_brightness;
        light.shadows_enabled = debug_state.settings.sun_shadows;
    }
}

fn spawn_debug_menu_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    debug_state: &DebugMenuState,
) {
    let page = current_page(debug_state);
    let entries = entries_for_page(debug_state, page);

    let root_node = Node {
        position_type: PositionType::Absolute,
        left: Val::Px(16.0),
        top: Val::Px(16.0),
        width: Val::Px(420.0),
        padding: UiRect::all(Val::Px(12.0)),
        row_gap: Val::Px(8.0),
        flex_direction: FlexDirection::Column,
        ..Default::default()
    };

    let root = commands
        .spawn((
            root_node,
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
            GlobalZIndex(200),
            DebugMenuUiTag,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Text::new(format!("Debug Menu: {}", page_title(page))),
            TextFont {
                font: default_font(asset_server),
                font_size: 20.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            DebugMenuUiTag,
        ));

        parent.spawn((
            Text::new("Left click: select | Right click: back"),
            TextFont {
                font: default_font(asset_server),
                font_size: 12.0,
                ..Default::default()
            },
            TextColor(Color::srgba(0.8, 0.8, 0.8, 0.9)),
            DebugMenuUiTag,
        ));

        for entry in entries {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(30.0),
                        padding: UiRect::new(
                            Val::Px(10.0),
                            Val::Px(10.0),
                            Val::Px(6.0),
                            Val::Px(6.0),
                        ),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.85)),
                    DebugMenuButton {
                        action: entry.action,
                    },
                    DebugMenuUiTag,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(entry.label),
                        TextFont {
                            font: default_font(asset_server),
                            font_size: 16.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        DebugMenuUiTag,
                    ));
                });
        }
    });
}

fn current_page(state: &DebugMenuState) -> DebugMenuPage {
    state.stack.last().copied().unwrap_or(DebugMenuPage::Root)
}

fn entries_for_page(state: &DebugMenuState, page: DebugMenuPage) -> Vec<DebugMenuEntry> {
    let mut entries = Vec::new();
    match page {
        DebugMenuPage::Root => {
            entries.push(DebugMenuEntry {
                label: "Camera".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::Camera),
            });
            entries.push(DebugMenuEntry {
                label: "Render".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::Render),
            });
            entries.push(DebugMenuEntry {
                label: "Physics".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::Physics),
            });
            entries.push(DebugMenuEntry {
                label: "Sun".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::Sun),
            });
        }
        DebugMenuPage::Camera => {
            entries.push(DebugMenuEntry {
                label: format!("FOV: {:.1}", state.settings.fov_degrees),
                action: DebugMenuAction::Noop,
            });
            entries.push(DebugMenuEntry {
                label: "FOV +5".to_string(),
                action: DebugMenuAction::AdjustFov(5.0),
            });
            entries.push(DebugMenuEntry {
                label: "FOV -5".to_string(),
                action: DebugMenuAction::AdjustFov(-5.0),
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::Render => {
            entries.push(DebugMenuEntry {
                label: format!("Bloom: {}", on_off(state.settings.bloom_enabled)),
                action: DebugMenuAction::ToggleBloom,
            });
            entries.push(DebugMenuEntry {
                label: format!("Fog: {}", on_off(state.settings.fog_enabled)),
                action: DebugMenuAction::ToggleFog,
            });
            entries.push(DebugMenuEntry {
                label: format!("DLSS: {}", on_off(state.settings.dlss_enabled)),
                action: DebugMenuAction::ToggleDlss,
            });
            entries.push(DebugMenuEntry {
                label: format!("Ray Tracing: {}", on_off(state.settings.ray_tracing_enabled)),
                action: DebugMenuAction::ToggleRayTracing,
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::Physics => {
            entries.push(DebugMenuEntry {
                label: format!("Gravity Y: {:.2}", state.settings.gravity.y),
                action: DebugMenuAction::Noop,
            });
            entries.push(DebugMenuEntry {
                label: "Gravity +1".to_string(),
                action: DebugMenuAction::AdjustGravity(1.0),
            });
            entries.push(DebugMenuEntry {
                label: "Gravity -1".to_string(),
                action: DebugMenuAction::AdjustGravity(-1.0),
            });
            entries.push(DebugMenuEntry {
                label: format!("Physics: {}", on_off(state.settings.physics_enabled)),
                action: DebugMenuAction::TogglePhysics,
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::Sun => {
            if state.settings.sun_present {
                entries.push(DebugMenuEntry {
                    label: format!("Brightness: {:.0}", state.settings.sun_brightness),
                    action: DebugMenuAction::Noop,
                });
                entries.push(DebugMenuEntry {
                    label: "Brightness +500".to_string(),
                    action: DebugMenuAction::AdjustSunBrightness(500.0),
                });
                entries.push(DebugMenuEntry {
                    label: "Brightness -500".to_string(),
                    action: DebugMenuAction::AdjustSunBrightness(-500.0),
                });
                entries.push(DebugMenuEntry {
                    label: format!("Shadows: {}", on_off(state.settings.sun_shadows)),
                    action: DebugMenuAction::ToggleSunShadows,
                });
            } else {
                entries.push(DebugMenuEntry {
                    label: "No sun in this scene".to_string(),
                    action: DebugMenuAction::Noop,
                });
            }
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
    }
    entries
}

fn page_title(page: DebugMenuPage) -> &'static str {
    match page {
        DebugMenuPage::Root => "Root",
        DebugMenuPage::Camera => "Camera",
        DebugMenuPage::Render => "Render",
        DebugMenuPage::Physics => "Physics",
        DebugMenuPage::Sun => "Sun",
    }
}

fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}

fn default_font(asset_server: &AssetServer) -> Handle<Font> {
    let _ = asset_server;
    Handle::<Font>::default()
}

fn safe_despawn(commands: &mut Commands, entity: Entity) {
    if let Ok(mut target) = commands.get_entity(entity) {
        target.despawn();
    }
}
