use bevy::ecs::system::entity_command;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::time::{Time, Virtual};
use bevy::ui::ComputedNode;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window};
use bevy::post_process::bloom::Bloom;
use bevy::pbr::{DistanceFog, FogFalloff};
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

#[derive(Component, Clone)]
pub(crate) struct DebugMenuSlider {
    kind: DebugMenuSliderKind,
    min: f32,
    max: f32,
    fill: Entity,
}

#[derive(Component, Clone)]
pub(crate) struct DebugMenuSliderLabel {
    kind: DebugMenuSliderKind,
    label: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DebugMenuSliderKind {
    Fov,
    GravityY,
    SunBrightness,
    DlssSharpness,
    BloomIntensity,
    BloomThreshold,
    BloomThresholdSoftness,
    FogAlpha,
    FogDensity,
    FogLinearStart,
    FogLinearEnd,
}

#[derive(Clone)]
enum DebugMenuAction {
    Noop,
    Open(DebugMenuPage),
    Back,
    ToggleBloom,
    ToggleFog,
    ToggleDlss,
    CycleDlssMode,
    CycleFogMode,
    ToggleRayTracing,
    CycleRayTracingMode,
    TogglePhysics,
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
    mut slider_params: ParamSet<(
        Query<(Entity, &Interaction, &DebugMenuSlider, &ComputedNode, &GlobalTransform)>,
        Query<&mut Node>,
        Query<(&mut Text, &DebugMenuSliderLabel)>,
    )>,
    mut text_colors: Query<&mut TextColor>,
    mut projections: Query<&mut Projection, With<SceneCamera>>,
    mut zoom_state: Option<ResMut<ZoomState>>,
    camera_entities: Query<Entity, With<SceneCamera>>,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    mut sun: Query<&mut DirectionalLight, With<SunLight>>,
    windows: Query<&Window, With<PrimaryWindow>>,
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
                &camera_entities,
                &mut rapier_config,
                &mut sun,
            );
        }
    }

    handle_slider_input(
        &mouse_buttons,
        &windows,
        &mut debug_state,
        &mut slider_params,
        &mut commands,
        &mut projections,
        &mut zoom_state,
        &camera_entities,
        &mut rapier_config,
        &mut sun,
    );

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

    debug_state.active_slider = None;
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
        if let Some(bloom) = bloom {
            debug_state.settings.bloom_intensity = bloom.intensity;
            debug_state.settings.bloom_threshold = bloom.prefilter.threshold;
            debug_state.settings.bloom_threshold_softness = bloom.prefilter.threshold_softness;
        }
        debug_state.settings.fog_enabled = fog.is_some();
        debug_state.settings.fog = fog.cloned();
        if let Some(fog) = fog {
            debug_state.settings.fog_alpha = fog.color.alpha();
            match &fog.falloff {
                FogFalloff::Linear { start, end } => {
                    debug_state.settings.fog_mode = "linear".to_string();
                    debug_state.settings.fog_linear_start = *start;
                    debug_state.settings.fog_linear_end = *end;
                }
                FogFalloff::Exponential { density } => {
                    debug_state.settings.fog_mode = "exponential".to_string();
                    debug_state.settings.fog_density = *density;
                }
                FogFalloff::ExponentialSquared { density } => {
                    debug_state.settings.fog_mode = "exponential_squared".to_string();
                    debug_state.settings.fog_density = *density;
                }
                FogFalloff::Atmospheric { .. } => {
                    debug_state.settings.fog_mode = "linear".to_string();
                }
            }
        }
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
            apply_bloom_settings(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleFog => {
            debug_state.settings.fog_enabled = !debug_state.settings.fog_enabled;
            apply_fog_settings(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleDlss => {
            debug_state.settings.dlss_enabled = !debug_state.settings.dlss_enabled;
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::CycleDlssMode => {
            debug_state.settings.dlss_mode = next_quality_mode(&debug_state.settings.dlss_mode);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::CycleFogMode => {
            debug_state.settings.fog_mode = next_fog_mode(&debug_state.settings.fog_mode);
            apply_fog_settings(debug_state, commands, camera_entities);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::ToggleRayTracing => {
            debug_state.settings.ray_tracing_enabled = !debug_state.settings.ray_tracing_enabled;
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::CycleRayTracingMode => {
            debug_state.settings.ray_tracing_mode =
                next_quality_mode(&debug_state.settings.ray_tracing_mode);
            debug_state.needs_refresh = true;
        }
        DebugMenuAction::TogglePhysics => {
            debug_state.settings.physics_enabled = !debug_state.settings.physics_enabled;
            apply_physics_toggle(debug_state, rapier_config);
            debug_state.needs_refresh = true;
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

fn apply_bloom_settings(
    debug_state: &DebugMenuState,
    commands: &mut Commands,
    cameras: &Query<Entity, With<SceneCamera>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    if debug_state.settings.bloom_enabled {
        let mut bloom = debug_state
            .settings
            .bloom
            .clone()
            .unwrap_or_else(Bloom::default);
        bloom.intensity = debug_state.settings.bloom_intensity;
        bloom.prefilter.threshold = debug_state.settings.bloom_threshold;
        bloom.prefilter.threshold_softness = debug_state.settings.bloom_threshold_softness;
        commands.entity(camera).insert(bloom);
    } else {
        commands.entity(camera).remove::<Bloom>();
    }
}

fn apply_fog_settings(
    debug_state: &DebugMenuState,
    commands: &mut Commands,
    cameras: &Query<Entity, With<SceneCamera>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    if debug_state.settings.fog_enabled {
        let mut fog = debug_state
            .settings
            .fog
            .clone()
            .unwrap_or_else(DistanceFog::default);
        let alpha = debug_state.settings.fog_alpha.clamp(0.0, 1.0);
        fog.color.set_alpha(alpha);
        fog.falloff = match debug_state.settings.fog_mode.as_str() {
            "exponential" => FogFalloff::Exponential {
                density: debug_state.settings.fog_density,
            },
            "exponential_squared" => FogFalloff::ExponentialSquared {
                density: debug_state.settings.fog_density,
            },
            _ => FogFalloff::Linear {
                start: debug_state.settings.fog_linear_start,
                end: debug_state.settings.fog_linear_end,
            },
        };
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

        for slider in sliders_for_page(debug_state, page) {
            let percent = ((slider.value - slider.min) / (slider.max - slider.min)).clamp(0.0, 1.0);
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(34.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..Default::default()
                    },
                    DebugMenuUiTag,
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(format!("{}: {:.2}", slider.label, slider.value)),
                        TextFont {
                            font: default_font(asset_server),
                            font_size: 14.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        DebugMenuSliderLabel {
                            kind: slider.kind,
                            label: slider.label,
                        },
                        DebugMenuUiTag,
                    ));

                    let mut fill_entity = None;
                    let mut bar = row.spawn((
                        Button,
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(12.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
                        DebugMenuUiTag,
                    ));
                    bar.with_children(|bar_builder| {
                        let fill = bar_builder
                            .spawn((
                                Node {
                                    width: Val::Percent(percent * 100.0),
                                    height: Val::Percent(100.0),
                                    ..Default::default()
                                },
                                BackgroundColor(Color::srgba(0.7, 0.7, 0.7, 0.9)),
                                DebugMenuUiTag,
                            ))
                            .id();
                        fill_entity = Some(fill);
                    });
                    if let Some(fill) = fill_entity {
                        bar.insert(DebugMenuSlider {
                            kind: slider.kind,
                            min: slider.min,
                            max: slider.max,
                            fill,
                        });
                    }
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
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::Render => {
            entries.push(DebugMenuEntry {
                label: "Bloom...".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::RenderBloom),
            });
            entries.push(DebugMenuEntry {
                label: "Fog...".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::RenderFog),
            });
            entries.push(DebugMenuEntry {
                label: "DLSS...".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::RenderDlss),
            });
            entries.push(DebugMenuEntry {
                label: "Ray Tracing...".to_string(),
                action: DebugMenuAction::Open(DebugMenuPage::RenderRayTracing),
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::RenderDlss => {
            entries.push(DebugMenuEntry {
                label: format!("DLSS: {}", on_off(state.settings.dlss_enabled)),
                action: DebugMenuAction::ToggleDlss,
            });
            entries.push(DebugMenuEntry {
                label: format!("Mode: {}", state.settings.dlss_mode),
                action: DebugMenuAction::CycleDlssMode,
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::RenderBloom => {
            entries.push(DebugMenuEntry {
                label: format!("Bloom: {}", on_off(state.settings.bloom_enabled)),
                action: DebugMenuAction::ToggleBloom,
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::RenderFog => {
            entries.push(DebugMenuEntry {
                label: format!("Fog: {}", on_off(state.settings.fog_enabled)),
                action: DebugMenuAction::ToggleFog,
            });
            entries.push(DebugMenuEntry {
                label: format!("Mode: {}", state.settings.fog_mode),
                action: DebugMenuAction::CycleFogMode,
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::RenderRayTracing => {
            entries.push(DebugMenuEntry {
                label: format!(
                    "Ray Tracing: {}",
                    on_off(state.settings.ray_tracing_enabled)
                ),
                action: DebugMenuAction::ToggleRayTracing,
            });
            entries.push(DebugMenuEntry {
                label: format!("Mode: {}", state.settings.ray_tracing_mode),
                action: DebugMenuAction::CycleRayTracingMode,
            });
            entries.push(DebugMenuEntry {
                label: "Back".to_string(),
                action: DebugMenuAction::Back,
            });
        }
        DebugMenuPage::Physics => {
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
        DebugMenuPage::RenderDlss => "DLSS",
        DebugMenuPage::RenderBloom => "Bloom",
        DebugMenuPage::RenderFog => "Fog",
        DebugMenuPage::RenderRayTracing => "Ray Tracing",
        DebugMenuPage::Physics => "Physics",
        DebugMenuPage::Sun => "Sun",
    }
}

fn on_off(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
}

fn next_quality_mode(current: &str) -> String {
    match current.trim().to_ascii_lowercase().as_str() {
        "performance" => "balanced".to_string(),
        "balanced" => "quality".to_string(),
        _ => "performance".to_string(),
    }
}

fn next_fog_mode(current: &str) -> String {
    match current.trim().to_ascii_lowercase().as_str() {
        "exponential" => "exponential_squared".to_string(),
        "exponential_squared" => "linear".to_string(),
        _ => "exponential".to_string(),
    }
}

struct DebugMenuSliderConfig {
    label: String,
    kind: DebugMenuSliderKind,
    min: f32,
    max: f32,
    value: f32,
}

impl DebugMenuSliderConfig {
    fn new(label: &str, kind: DebugMenuSliderKind, min: f32, max: f32, value: f32) -> Self {
        Self {
            label: label.to_string(),
            kind,
            min,
            max,
            value,
        }
    }
}

fn sliders_for_page(
    state: &DebugMenuState,
    page: DebugMenuPage,
) -> Vec<DebugMenuSliderConfig> {
    let mut sliders = Vec::new();
    match page {
        DebugMenuPage::Camera => {
            sliders.push(DebugMenuSliderConfig::new(
                "FOV",
                DebugMenuSliderKind::Fov,
                30.0,
                160.0,
                state.settings.fov_degrees,
            ));
        }
        DebugMenuPage::Physics => {
            sliders.push(DebugMenuSliderConfig::new(
                "Gravity Y",
                DebugMenuSliderKind::GravityY,
                -30.0,
                10.0,
                state.settings.gravity.y,
            ));
        }
        DebugMenuPage::Sun => {
            if state.settings.sun_present {
                sliders.push(DebugMenuSliderConfig::new(
                    "Brightness",
                    DebugMenuSliderKind::SunBrightness,
                    0.0,
                    20000.0,
                    state.settings.sun_brightness,
                ));
            }
        }
        DebugMenuPage::RenderDlss => {
            sliders.push(DebugMenuSliderConfig::new(
                "Sharpness",
                DebugMenuSliderKind::DlssSharpness,
                0.0,
                1.0,
                state.settings.dlss_sharpness,
            ));
        }
        DebugMenuPage::RenderBloom => {
            sliders.push(DebugMenuSliderConfig::new(
                "Intensity",
                DebugMenuSliderKind::BloomIntensity,
                0.0,
                1.0,
                state.settings.bloom_intensity,
            ));
            sliders.push(DebugMenuSliderConfig::new(
                "Threshold",
                DebugMenuSliderKind::BloomThreshold,
                0.0,
                2.0,
                state.settings.bloom_threshold,
            ));
            sliders.push(DebugMenuSliderConfig::new(
                "Softness",
                DebugMenuSliderKind::BloomThresholdSoftness,
                0.0,
                1.0,
                state.settings.bloom_threshold_softness,
            ));
        }
        DebugMenuPage::RenderFog => {
            sliders.push(DebugMenuSliderConfig::new(
                "Alpha",
                DebugMenuSliderKind::FogAlpha,
                0.0,
                1.0,
                state.settings.fog_alpha,
            ));
            sliders.push(DebugMenuSliderConfig::new(
                "Density",
                DebugMenuSliderKind::FogDensity,
                0.0,
                0.2,
                state.settings.fog_density,
            ));
            sliders.push(DebugMenuSliderConfig::new(
                "Start",
                DebugMenuSliderKind::FogLinearStart,
                0.0,
                50.0,
                state.settings.fog_linear_start,
            ));
            sliders.push(DebugMenuSliderConfig::new(
                "End",
                DebugMenuSliderKind::FogLinearEnd,
                10.0,
                200.0,
                state.settings.fog_linear_end,
            ));
        }
        _ => {}
    }
    sliders
}

fn handle_slider_input(
    mouse_buttons: &ButtonInput<MouseButton>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    debug_state: &mut DebugMenuState,
    slider_params: &mut ParamSet<(
        Query<(Entity, &Interaction, &DebugMenuSlider, &ComputedNode, &GlobalTransform)>,
        Query<&mut Node>,
        Query<(&mut Text, &DebugMenuSliderLabel)>,
    )>,
    commands: &mut Commands,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
    camera_entities: &Query<Entity, With<SceneCamera>>,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    if mouse_buttons.just_released(MouseButton::Left) {
        debug_state.active_slider = None;
    }

    let mut updates = Vec::new();
    {
        let mut sliders = slider_params.p0();
        for (entity, interaction, slider, computed, transform) in sliders.iter_mut() {
            if mouse_buttons.just_pressed(MouseButton::Left) && *interaction == Interaction::Hovered {
                debug_state.active_slider = Some(entity);
            }

            if debug_state.active_slider != Some(entity) {
                continue;
            }

            let size = computed.size;
            let center = transform.translation().truncate();
            let left = center.x - size.x / 2.0;
            let ratio = ((cursor.x - left) / size.x).clamp(0.0, 1.0);
            let value = slider.min + (slider.max - slider.min) * ratio;

            apply_slider_value(
                slider.kind.clone(),
                value,
                debug_state,
                commands,
                projections,
                zoom_state,
                camera_entities,
                rapier_config,
                sun,
            );
            updates.push((slider.fill, slider.kind, ratio, value));
        }
    }

    if updates.is_empty() {
        return;
    }

    {
        let mut slider_fills = slider_params.p1();
        for (fill_entity, _kind, ratio, _value) in &updates {
            if let Ok(mut fill) = slider_fills.get_mut(*fill_entity) {
                fill.width = Val::Percent(ratio * 100.0);
            }
        }
    }

    {
        let mut slider_labels = slider_params.p2();
        for (_fill_entity, kind, _ratio, value) in updates {
            update_slider_label(&mut slider_labels, kind, value);
        }
    }
}

fn update_slider_label(
    slider_labels: &mut Query<(&mut Text, &DebugMenuSliderLabel)>,
    kind: DebugMenuSliderKind,
    value: f32,
) {
    for (mut text, label) in slider_labels.iter_mut() {
        if label.kind == kind {
            text.0 = format!("{}: {:.2}", label.label, value);
            return;
        }
    }
}

fn apply_slider_value(
    kind: DebugMenuSliderKind,
    value: f32,
    debug_state: &mut DebugMenuState,
    commands: &mut Commands,
    projections: &mut Query<&mut Projection, With<SceneCamera>>,
    zoom_state: &mut Option<ResMut<ZoomState>>,
    camera_entities: &Query<Entity, With<SceneCamera>>,
    rapier_config: &mut Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
    sun: &mut Query<&mut DirectionalLight, With<SunLight>>,
) {
    match kind {
        DebugMenuSliderKind::Fov => {
            debug_state.settings.fov_degrees = value;
            apply_fov(debug_state, projections, zoom_state);
        }
        DebugMenuSliderKind::GravityY => {
            debug_state.settings.gravity.y = value;
            apply_gravity(debug_state, rapier_config);
        }
        DebugMenuSliderKind::SunBrightness => {
            debug_state.settings.sun_brightness = value;
            apply_sun(debug_state, sun);
        }
        DebugMenuSliderKind::DlssSharpness => {
            debug_state.settings.dlss_sharpness = value;
        }
        DebugMenuSliderKind::BloomIntensity => {
            debug_state.settings.bloom_intensity = value;
            apply_bloom_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::BloomThreshold => {
            debug_state.settings.bloom_threshold = value;
            apply_bloom_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::BloomThresholdSoftness => {
            debug_state.settings.bloom_threshold_softness = value;
            apply_bloom_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogAlpha => {
            debug_state.settings.fog_alpha = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogDensity => {
            debug_state.settings.fog_density = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogLinearStart => {
            debug_state.settings.fog_linear_start = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
        DebugMenuSliderKind::FogLinearEnd => {
            debug_state.settings.fog_linear_end = value;
            apply_fog_settings(debug_state, commands, camera_entities);
        }
    }
}

fn default_font(asset_server: &AssetServer) -> Handle<Font> {
    let _ = asset_server;
    Handle::<Font>::default()
}

fn safe_despawn(commands: &mut Commands, entity: Entity) {
    commands
        .entity(entity)
        .queue_silenced(entity_command::despawn());
}
