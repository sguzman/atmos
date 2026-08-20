use bevy::prelude::*;

use super::components::{Agent, AgentMotion, Food, FoodTarget, Hunger, Perception};

pub fn increase_hunger(time: Res<Time>, mut agents: Query<&mut Hunger, With<Agent>>) {
    for mut hunger in &mut agents {
        hunger.level = (hunger.level + hunger.rate_per_second * time.delta_secs()).min(hunger.max);
    }
}

pub fn choose_food_target(
    foods: Query<(Entity, &Transform), With<Food>>,
    mut agents: Query<(&Transform, &Hunger, &Perception, &mut FoodTarget), With<Agent>>,
) {
    for (agent_transform, hunger, perception, mut target) in &mut agents {
        if hunger.level < hunger.seek_threshold {
            target.0 = None;
            continue;
        }

        if let Some(current) = target.0 {
            if foods.get(current).is_ok() {
                continue;
            }
            target.0 = None;
        }

        let radius_squared = perception.radius * perception.radius;
        let mut best: Option<(Entity, f32)> = None;

        for (food_entity, food_transform) in &foods {
            let delta = food_transform.translation - agent_transform.translation;
            let horizontal_distance_squared = delta.x * delta.x + delta.z * delta.z;
            if horizontal_distance_squared > radius_squared {
                continue;
            }

            if best.is_none_or(|(_, distance)| horizontal_distance_squared < distance) {
                best = Some((food_entity, horizontal_distance_squared));
            }
        }

        if let Some((food_entity, _)) = best {
            target.0 = Some(food_entity);
            info!(
                ?food_entity,
                hunger = hunger.level,
                "agent selected food target"
            );
        }
    }
}

pub fn move_toward_food(
    time: Res<Time>,
    foods: Query<&Transform, With<Food>>,
    mut agents: Query<(&mut Transform, &AgentMotion, &FoodTarget), With<Agent>>,
) {
    for (mut transform, motion, target) in &mut agents {
        let Some(food_entity) = target.0 else {
            continue;
        };
        let Ok(food_transform) = foods.get(food_entity) else {
            continue;
        };

        let delta = food_transform.translation - transform.translation;
        let horizontal = Vec3::new(delta.x, 0.0, delta.z);
        let distance = horizontal.length();
        if distance <= motion.consume_distance || distance <= f32::EPSILON {
            continue;
        }

        let step = (motion.speed * time.delta_secs()).min(distance - motion.consume_distance);
        transform.translation += horizontal / distance * step;
    }
}

pub fn consume_food(
    mut commands: Commands,
    foods: Query<(&Transform, &Food)>,
    mut agents: Query<(&Transform, &AgentMotion, &mut Hunger, &mut FoodTarget), With<Agent>>,
) {
    for (agent_transform, motion, mut hunger, mut target) in &mut agents {
        let Some(food_entity) = target.0 else {
            continue;
        };
        let Ok((food_transform, food)) = foods.get(food_entity) else {
            target.0 = None;
            continue;
        };

        let delta = food_transform.translation - agent_transform.translation;
        let horizontal_distance = Vec2::new(delta.x, delta.z).length();
        if horizontal_distance > motion.consume_distance {
            continue;
        }

        hunger.level = (hunger.level - food.nutrition).max(0.0);
        commands.entity(food_entity).despawn();
        target.0 = None;
        info!(
            hunger = hunger.level,
            nutrition = food.nutrition,
            "agent consumed food"
        );
    }
}
