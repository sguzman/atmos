use bevy::prelude::*;

use crate::scenes::input::SceneInputConfig;

use super::super::overlay::OverlayTag;

pub(crate) fn toggle_overlays(
    keys: Res<ButtonInput<KeyCode>>,
    config: Option<
        Res<SceneInputConfig>,
    >,
    mut overlays: Query<(
        &OverlayTag,
        &mut Visibility,
    )>,
) {
    let Some(config) = config else {
        return;
    };
    for overlay in &config.overlays {
        let Some(key) = overlay.toggle
        else {
            continue;
        };
        if keys.just_pressed(key) {
            for (_tag, mut vis) in overlays
                .iter_mut()
                .filter(|(tag, _)| tag.name == overlay.name)
            {
                vis.toggle_visible_hidden();
            }
        }
    }
}
