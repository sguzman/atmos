use bevy::{
    prelude::*,
    ui::{PositionType, UiTransform, Val, Val2},
};

use crate::scenes::config::{parse_color, OverlayAnchor, OverlayElement, TextOverlay};
use crate::scenes::loaders::load_overlay_config;

pub fn spawn_overlays(mut commands: Commands, asset_server: Res<AssetServer>) {
    let overlay = load_overlay_config("debug");

    for element in overlay.elements {
        match element {
            OverlayElement::Text(text) => spawn_text_overlay(&mut commands, &asset_server, text),
            OverlayElement::Image(_img) => {
                // Image overlays can be added here later
            }
        }
    }
}

fn spawn_text_overlay(commands: &mut Commands, asset_server: &AssetServer, text: TextOverlay) {
    let color = parse_color(&text.color).unwrap_or([255, 255, 255]);
    let color = Color::srgb_u8(color[0], color[1], color[2]);
    let node = node_from_anchor(&text.common.anchor);
    let translation = Val2::new(Val::Px(text.common.offset.x), Val::Px(text.common.offset.y));
    let transform = UiTransform {
        translation,
        rotation: Rot2::degrees(text.common.rotation_deg),
        scale: Vec2::splat(text.common.scale.max(f32::EPSILON)),
    };

    let visibility = if text.common.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    commands.spawn((
        node,
        transform,
        GlobalZIndex(100), // keep overlays on top
        Text::new(text.content),
        TextFont {
            font: default_font(asset_server),
            font_size: text.font_size,
            ..default()
        },
        TextColor(color),
        visibility,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn node_from_anchor(anchor: &OverlayAnchor) -> Node {
    let mut node = Node {
        position_type: PositionType::Absolute,
        ..default()
    };
    match anchor {
        OverlayAnchor::TopLeft => {
            node.top = Val::Px(0.0);
            node.left = Val::Px(0.0);
        }
        OverlayAnchor::TopRight => {
            node.top = Val::Px(0.0);
            node.right = Val::Px(0.0);
        }
        OverlayAnchor::BottomLeft => {
            node.bottom = Val::Px(0.0);
            node.left = Val::Px(0.0);
        }
        OverlayAnchor::BottomRight => {
            node.bottom = Val::Px(0.0);
            node.right = Val::Px(0.0);
        }
        OverlayAnchor::Top => {
            node.top = Val::Px(0.0);
            node.left = Val::Percent(50.0);
        }
        OverlayAnchor::Bottom => {
            node.bottom = Val::Px(0.0);
            node.left = Val::Percent(50.0);
        }
        OverlayAnchor::Left => {
            node.left = Val::Px(0.0);
            node.top = Val::Percent(50.0);
        }
        OverlayAnchor::Right => {
            node.right = Val::Px(0.0);
            node.top = Val::Percent(50.0);
        }
        OverlayAnchor::Center => {
            node.left = Val::Percent(50.0);
            node.top = Val::Percent(50.0);
        }
    }
    node
}

fn default_font(asset_server: &AssetServer) -> Handle<Font> {
    // Bevy ships with a fallback font; the default handle resolves to it.
    let _ = asset_server; // keep signature in case asset loading is needed later
    Handle::<Font>::default()
}
