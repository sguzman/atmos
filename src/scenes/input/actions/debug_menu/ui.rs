use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::scenes::input::{
    DebugMenuPage, DebugMenuState,
};

use super::types::{
    DebugMenuAction,
    DebugMenuAdjustStep,
    DebugMenuButton, DebugMenuEntry,
    DebugMenuSliderConfig,
    DebugMenuSliderKind,
    DebugMenuUiTag,
};

pub(crate) fn spawn_debug_menu_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    debug_state: &DebugMenuState,
) {
    let page =
        current_page(debug_state);
    let entries = entries_for_page(
        debug_state,
        page,
    );

    let container = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            GlobalZIndex(200),
            DebugMenuUiTag,
        ))
        .id();

    commands.entity(container).with_children(|parent| {
        parent
            .spawn((
                Node {
                    width: Val::Px(520.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(10.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
                DebugMenuUiTag,
            ))
            .with_children(|parent| {
        parent.spawn((
            Text::new(format!("Debug Menu: {}", page_title(page))),
            TextFont {
                font: default_font(asset_server),
                font_size: 22.0,
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
                    Interaction::default(),
                    FocusPolicy::Block,
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
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
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
                        DebugMenuUiTag,
                    ));

                    row.spawn((
                        Node {
                            width: Val::Px(260.0),
                            height: Val::Px(28.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..Default::default()
                        },
                        DebugMenuUiTag,
                    ))
                    .with_children(|buttons| {
                        let mut add_button = |label: &str, step: DebugMenuAdjustStep| {
                            buttons
                                .spawn((
                                    Button,
                                    Interaction::default(),
                                    FocusPolicy::Block,
                                    Node {
                                        width: Val::Px(58.0),
                                        height: Val::Px(24.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.85)),
                                    DebugMenuButton {
                                        action: DebugMenuAction::AdjustSlider {
                                            kind: slider.kind,
                                            min: slider.min,
                                            max: slider.max,
                                            step,
                                        },
                                    },
                                    DebugMenuUiTag,
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new(label),
                                        TextFont {
                                            font: default_font(asset_server),
                                            font_size: 12.0,
                                            ..Default::default()
                                        },
                                        TextColor(Color::WHITE),
                                        DebugMenuUiTag,
                                    ));
                                });
                        };

                        add_button("min", DebugMenuAdjustStep::Min);
                        add_button("-10%", DebugMenuAdjustStep::Minus);
                        add_button("+10%", DebugMenuAdjustStep::Plus);
                        add_button("max", DebugMenuAdjustStep::Max);
                    });
                });
        }
            });
    });
}

fn current_page(
    state: &DebugMenuState,
) -> DebugMenuPage {
    state
        .stack
        .last()
        .copied()
        .unwrap_or(DebugMenuPage::Root)
}

fn entries_for_page(
    state: &DebugMenuState,
    page: DebugMenuPage,
) -> Vec<DebugMenuEntry> {
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
                label: format!("Ray Tracing: {}", on_off(state.settings.ray_tracing_enabled)),
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

fn page_title(
    page: DebugMenuPage,
) -> &'static str {
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
            if state
                .settings
                .sun_present
            {
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

fn default_font(
    asset_server: &AssetServer,
) -> Handle<Font> {
    let _ = asset_server;
    Handle::<Font>::default()
}
