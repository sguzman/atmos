use bevy::prelude::{
    ButtonInput, Color, Commands, Component, Entity, GlobalZIndex, KeyCode, Node, Query, Res,
    ResMut, Text, TextColor, TextFont, UiTransform, Val, Val2, Vec2, With,
};

use crate::scenes::TomlAsset;
use crate::scenes::config::DialogueConfig;
use crate::scenes::input::{ActionStates, DialogueState, SceneDialogueConfig};
use crate::scenes::loaders::{ConfigLoad, TomlCache, load_dialogue_config};
use crate::scenes::spawn::OverlayTag;

#[derive(Component)]
pub struct DialogueUiTag;

pub fn apply_dialogue_action(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<Res<SceneDialogueConfig>>,
    states: Option<Res<ActionStates>>,
    dialogue_state: Option<ResMut<DialogueState>>,
    mut commands: Commands,
    ui_nodes: Query<Entity, With<DialogueUiTag>>,
    mut overlays: Query<(&OverlayTag, &mut bevy::prelude::Visibility)>,
    asset_server: Res<bevy::prelude::AssetServer>,
    toml_assets: Res<bevy::prelude::Assets<TomlAsset>>,
    mut toml_cache: ResMut<TomlCache>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut dialogue_state) = dialogue_state else {
        return;
    };

    let in_range = states.get(&config.prompt_action_id).pressed;
    let interact_pressed = states.get(&config.interact_action_id).just_pressed;

    for (_tag, mut vis) in overlays
        .iter_mut()
        .filter(|(tag, _)| tag.name == config.prompt_overlay)
    {
        *vis = if in_range && !dialogue_state.active {
            bevy::prelude::Visibility::Visible
        } else {
            bevy::prelude::Visibility::Hidden
        };
    }

    if !dialogue_state.active {
        if interact_pressed && in_range {
            dialogue_state.pending = true;
        }

        if dialogue_state.pending {
            match load_dialogue_config(
                &config.dialogue,
                &mut toml_cache,
                &asset_server,
                &toml_assets,
            ) {
                ConfigLoad::Pending => {
                    return;
                }
                ConfigLoad::Ready(dialogue) => {
                    if dialogue.start.trim().is_empty() {
                        return;
                    }
                    dialogue_state.active = true;
                    dialogue_state.pending = false;
                    dialogue_state.current = dialogue.start.clone();
                    dialogue_state.dialogue = Some(dialogue);
                }
            }
        } else {
            return;
        }
    }

    let (node_text, node_options) = {
        let Some(dialogue) = dialogue_state.dialogue.as_ref() else {
            return;
        };
        let Some(node) = find_node(dialogue, &dialogue_state.current) else {
            dialogue_state.active = false;
            return;
        };
        (node.text.clone(), node.options.clone())
    };

    if !node_options.is_empty() {
        if let Some(index) = resolve_option_index(&keys, &config.option_keys) {
            if index < node_options.len() {
                let option = &node_options[index];
                if option.once && dialogue_state.visited.contains(&option.id) {
                    return;
                }
                dialogue_state.visited.insert(option.id.clone());
                if let Some(next) = option.next.as_ref() {
                    if !next.trim().is_empty() {
                        dialogue_state.current = next.clone();
                    }
                } else {
                    dialogue_state.active = false;
                }
            }
        }
    } else if interact_pressed {
        dialogue_state.active = false;
    }

    if !dialogue_state.active {
        for entity in ui_nodes.iter() {
            commands
                .entity(entity)
                .queue_silenced(bevy::ecs::system::entity_command::despawn());
        }
        return;
    }

    for entity in ui_nodes.iter() {
        commands
            .entity(entity)
            .queue_silenced(bevy::ecs::system::entity_command::despawn());
    }

    spawn_dialogue_ui(
        &mut commands,
        &asset_server,
        &node_text,
        &node_options,
        &config.option_labels,
        dialogue_state.visited.clone(),
    );
}

fn find_node<'a>(
    dialogue: &'a DialogueConfig,
    id: &str,
) -> Option<&'a crate::scenes::config::DialogueNode> {
    dialogue.nodes.iter().find(|node| node.id == id)
}

fn resolve_option_index(keys: &ButtonInput<KeyCode>, option_keys: &[KeyCode]) -> Option<usize> {
    for (index, key) in option_keys.iter().enumerate() {
        if keys.just_pressed(*key) {
            return Some(index);
        }
    }
    None
}

fn spawn_dialogue_ui(
    commands: &mut Commands,
    asset_server: &bevy::prelude::AssetServer,
    node_text: &str,
    node_options: &[crate::scenes::config::DialogueOption],
    option_labels: &[String],
    visited: std::collections::HashSet<String>,
) {
    let mut body = String::new();
    body.push_str(node_text);
    body.push_str("\n\n");
    for (idx, option) in node_options.iter().enumerate() {
        let key_label = option_labels
            .get(idx)
            .map(|label| label.as_str())
            .unwrap_or("?");
        let done = if visited.contains(&option.id) {
            " (done)"
        } else {
            ""
        };
        let line = format!("{key_label}. {}{}\n", option.text, done);
        body.push_str(&line);
    }

    let node_layout = Node {
        position_type: bevy::ui::PositionType::Absolute,
        ..Default::default()
    };
    let transform = UiTransform {
        translation: Val2::new(Val::Percent(50.0), Val::Percent(10.0)),
        rotation: Default::default(),
        scale: Vec2::splat(1.0),
    };

    commands.spawn((
        node_layout,
        transform,
        GlobalZIndex(110),
        Text::new(body),
        TextFont {
            font: default_font(asset_server),
            font_size: bevy::text::FontSize::Px(22.0),
            ..Default::default()
        },
        TextColor(Color::WHITE),
        DialogueUiTag,
    ));
}

fn default_font(asset_server: &bevy::prelude::AssetServer) -> bevy::text::FontSource {
    let _ = asset_server;
    bevy::text::FontSource::default()
}
