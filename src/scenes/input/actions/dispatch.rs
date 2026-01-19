use bevy::{
    input::{
        keyboard::KeyCode,
        mouse::MouseButton,
    },
    prelude::{
        ButtonInput, GlobalTransform,
        Query, Res, ResMut, With,
    },
};

use crate::scenes::input::PlayerBody;
use crate::scenes::input::{
    ActionStates, SceneActionTriggers,
    TriggerMode, TriggerSource,
    VolumeShapeKind, VolumeTriggerMode,
};

pub fn update_action_states(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<
        ButtonInput<MouseButton>,
    >,
    triggers: Option<
        ResMut<SceneActionTriggers>,
    >,
    player: Query<
        &GlobalTransform,
        With<PlayerBody>,
    >,
    states: Option<
        ResMut<ActionStates>,
    >,
) {
    let Some(mut states) = states
    else {
        return;
    };
    let Some(mut triggers) = triggers
    else {
        states.clear();
        return;
    };
    states.clear();

    for trigger in triggers.input.iter()
    {
        let (
            pressed,
            just_pressed,
            just_released,
        ) = match trigger.source {
            TriggerSource::Key(key) => {
                resolve_key_state(
                    &keys,
                    key,
                    trigger.mode,
                )
            }
            TriggerSource::Mouse(
                button,
            ) => resolve_mouse_state(
                &mouse,
                button,
                trigger.mode,
            ),
        };
        states.update(
            &trigger.action,
            pressed,
            just_pressed,
            just_released,
        );
    }

    let player_pos =
        match player.iter().next() {
            Some(transform) => {
                transform.translation()
            }
            None => return,
        };
    for trigger in
        triggers.volumes.iter_mut()
    {
        if trigger.once && trigger.fired
        {
            continue;
        }
        let inside = match trigger
            .shape
            .kind
        {
            VolumeShapeKind::Sphere => {
                (player_pos
                    - trigger.position)
                    .length()
                    <= trigger
                        .shape
                        .radius
            }
            VolumeShapeKind::Box => {
                let half =
                    trigger.shape.size
                        * 0.5;
                let delta = player_pos
                    - trigger.position;
                delta.x.abs() <= half.x
                    && delta.y.abs()
                        <= half.y
                    && delta.z.abs()
                        <= half.z
            }
        };

        let was_inside = trigger.inside;
        trigger.inside = inside;

        let (pressed, just_pressed, just_released) = match trigger.mode {
            VolumeTriggerMode::Enter => (inside && !was_inside, inside && !was_inside, false),
            VolumeTriggerMode::Exit => (!inside && was_inside, !inside && was_inside, false),
            VolumeTriggerMode::Inside => (inside, inside && !was_inside, !inside && was_inside),
        };

        if just_pressed && trigger.once
        {
            trigger.fired = true;
        }

        states.update(
            &trigger.action,
            pressed,
            just_pressed,
            just_released,
        );
    }
}

fn resolve_key_state(
    keys: &ButtonInput<KeyCode>,
    key: KeyCode,
    mode: TriggerMode,
) -> (bool, bool, bool) {
    match mode {
        TriggerMode::Press => {
            let pressed =
                keys.just_pressed(key);
            (pressed, pressed, false)
        }
        TriggerMode::Hold => (
            keys.pressed(key),
            keys.just_pressed(key),
            keys.just_released(key),
        ),
    }
}

fn resolve_mouse_state(
    mouse: &ButtonInput<MouseButton>,
    button: MouseButton,
    mode: TriggerMode,
) -> (bool, bool, bool) {
    match mode {
        TriggerMode::Press => {
            let pressed = mouse
                .just_pressed(button);
            (pressed, pressed, false)
        }
        TriggerMode::Hold => (
            mouse.pressed(button),
            mouse.just_pressed(button),
            mouse.just_released(button),
        ),
    }
}
