use bevy::prelude::*;

use super::components::{
    Agent, AgentBodyState, AgentBrain, AgentDebugText, AgentMotion, Food, Hunger, Intention,
    Perception, SimulationClock,
};
use super::config::{AgentDemoConfig, ScriptedEventAction};
use super::logic::{AgentStepInput, HungerState, food_world_state, increase_hunger, step_agent};

type AgentMindQueryItem<'a> = (
    &'a Transform,
    &'a Perception,
    &'a AgentMotion,
    &'a mut Hunger,
    &'a mut AgentBrain,
    &'a mut AgentBodyState,
);

pub fn advance_simulation_clock(time: Res<Time>, mut clock: ResMut<SimulationClock>) {
    clock.elapsed += time.delta_secs();
}

pub fn apply_scripted_events(
    mut commands: Commands,
    clock: Res<SimulationClock>,
    config: Res<AgentDemoConfig>,
    foods: Query<(Entity, &Food)>,
    mut fired_events: Local<Vec<usize>>,
) {
    for (index, event) in config.scripted_events.iter().enumerate() {
        if fired_events.contains(&index) || clock.elapsed < event.at_seconds {
            continue;
        }

        match &event.action {
            ScriptedEventAction::RemoveFood { food_id } => {
                for (entity, food) in &foods {
                    if &food.id == food_id {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }

        fired_events.push(index);
    }
}

pub fn update_agent_mind(
    foods: Query<(&Transform, &Food)>,
    mut agents: Query<AgentMindQueryItem<'_>, With<Agent>>,
    config: Res<AgentDemoConfig>,
) {
    let world_foods = foods
        .iter()
        .map(|(transform, food)| {
            let mut world = food_world_state(&super::config::FoodConfig {
                id: food.id.clone(),
                position: super::config::Vec3Config {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    z: transform.translation.z,
                },
                nutrition: food.nutrition,
            });
            world.available = true;
            world
        })
        .collect::<Vec<_>>();

    for (transform, perception, motion, mut hunger, mut brain, mut body_state) in &mut agents {
        let current_target = brain.mind.selected_target.clone();
        let result = step_agent(AgentStepInput {
            agent_position: transform.translation,
            hunger: HungerState {
                level: hunger.level,
                max: hunger.max,
                seek_threshold: hunger.seek_threshold,
            },
            perception_radius: perception.radius,
            consume_distance: motion.consume_distance,
            current_target,
            foods: world_foods.clone(),
            decision: config.agent.decision,
        });

        body_state.look_target = result.move_target.or_else(|| {
            result
                .mind
                .perceived
                .iter()
                .find(|food| Some(&food.id) == result.mind.selected_target.as_ref())
                .map(|food| food.position)
        });
        body_state.desired_velocity = match result.move_target {
            Some(target) => {
                let delta = target - transform.translation;
                Vec3::new(delta.x, 0.0, delta.z).normalize_or_zero() * motion.speed
            }
            None => Vec3::ZERO,
        };

        if let Some(hunger_after) = result.hunger_after_consumption {
            hunger.level = hunger_after;
        }
        brain.mind = result.mind;
    }
}

pub fn move_agent_body(
    time: Res<Time>,
    mut agents: Query<(&mut Transform, &AgentBodyState), With<Agent>>,
) {
    for (mut transform, body_state) in &mut agents {
        let velocity = body_state.desired_velocity;
        transform.translation += velocity * time.delta_secs();

        if let Some(look_target) = body_state.look_target {
            let facing = Vec3::new(
                look_target.x - transform.translation.x,
                0.0,
                look_target.z - transform.translation.z,
            );
            if facing.length_squared() > f32::EPSILON {
                let yaw = facing.x.atan2(-facing.z);
                transform.rotation = Quat::from_rotation_y(yaw);
            }
        }
    }
}

pub fn consume_selected_food(
    mut commands: Commands,
    foods: Query<(Entity, &Food)>,
    agents: Query<&AgentBrain, With<Agent>>,
) {
    for brain in &agents {
        let Intention::ConsumeFood { food_id } = &brain.mind.intention else {
            continue;
        };

        for (entity, food) in &foods {
            if &food.id == food_id {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn increase_agent_hunger(time: Res<Time>, mut agents: Query<&mut Hunger, With<Agent>>) {
    for mut hunger in &mut agents {
        hunger.level = increase_hunger(
            hunger.level,
            hunger.rate_per_second,
            hunger.max,
            time.delta_secs(),
        );
    }
}

pub fn update_debug_text(
    config: Res<AgentDemoConfig>,
    clock: Res<SimulationClock>,
    agent: Query<(&Hunger, &AgentBrain), With<Agent>>,
    mut texts: Query<&mut Text, With<AgentDebugText>>,
) {
    let Ok((hunger, brain)) = agent.single() else {
        return;
    };
    let Ok(mut text) = texts.single_mut() else {
        return;
    };

    let mut lines = vec![
        format!("Showcase: {}", config.showcase.name),
        config.showcase.description.clone(),
        format!("Time: {:.1}s", clock.elapsed),
        format!(
            "Hunger: {:.1}/{:.1} (seek at {:.1})",
            hunger.level, hunger.max, hunger.seek_threshold
        ),
        format!(
            "Selected target: {}",
            brain.mind.selected_target.as_deref().unwrap_or("none")
        ),
        format!("Intention: {}", intention_label(&brain.mind.intention)),
        String::new(),
        "Perceived food:".to_string(),
    ];

    if brain.mind.perceived.is_empty() {
        lines.push("  none".to_string());
    } else {
        for food in &brain.mind.perceived {
            lines.push(format!(
                "  {} dist={:.2} nutrition={:.1}",
                food.id, food.distance, food.nutrition
            ));
        }
    }

    lines.push(String::new());
    lines.push("Candidate scores:".to_string());
    if brain.mind.candidates.is_empty() {
        lines.push("  none".to_string());
    } else {
        for candidate in &brain.mind.candidates {
            lines.push(format!(
                "  {} score={:.2} hunger={:.2} nutrition={:.2} distance_penalty={:.2}",
                candidate.id,
                candidate.score,
                candidate.score_parts.hunger_pressure,
                candidate.score_parts.nutrition_value,
                candidate.score_parts.distance_penalty
            ));
        }
    }

    *text = Text::new(lines.join("\n"));
}

fn intention_label(intention: &Intention) -> String {
    match intention {
        Intention::Idle => "Idle".to_string(),
        Intention::SeekFood { food_id } => format!("SeekFood({food_id})"),
        Intention::ConsumeFood { food_id } => format!("ConsumeFood({food_id})"),
    }
}
