use bevy::prelude::Vec3;

use super::components::{AgentMind, CandidateScore, Intention, PerceivedFood, ScoreBreakdown};
use super::config::{DecisionConfig, FoodConfig};

#[derive(Debug, Clone, PartialEq)]
pub struct FoodWorldState {
    pub id: String,
    pub position: Vec3,
    pub nutrition: f32,
    pub available: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentStepInput {
    pub agent_position: Vec3,
    pub hunger: HungerState,
    pub perception_radius: f32,
    pub consume_distance: f32,
    pub current_target: Option<String>,
    pub foods: Vec<FoodWorldState>,
    pub decision: DecisionConfig,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HungerState {
    pub level: f32,
    pub max: f32,
    pub seek_threshold: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentStepResult {
    pub mind: AgentMind,
    pub move_target: Option<Vec3>,
    pub consumed_food: Option<String>,
    pub hunger_after_consumption: Option<f32>,
}

pub fn increase_hunger(level: f32, rate_per_second: f32, max: f32, delta_secs: f32) -> f32 {
    (level + rate_per_second * delta_secs).clamp(0.0, max)
}

pub fn perceive_foods(
    agent_position: Vec3,
    perception_radius: f32,
    foods: &[FoodWorldState],
) -> Vec<PerceivedFood> {
    let radius_squared = perception_radius * perception_radius;
    let mut perceived = foods
        .iter()
        .filter(|food| food.available)
        .filter_map(|food| {
            let delta = food.position - agent_position;
            let horizontal_distance_squared = delta.x * delta.x + delta.z * delta.z;
            if horizontal_distance_squared > radius_squared {
                return None;
            }
            Some(PerceivedFood {
                id: food.id.clone(),
                position: food.position,
                distance: horizontal_distance_squared.sqrt(),
                nutrition: food.nutrition,
            })
        })
        .collect::<Vec<_>>();

    perceived.sort_by(|left, right| left.distance.total_cmp(&right.distance));
    perceived
}

pub fn score_candidates(
    hunger: HungerState,
    perceived: &[PerceivedFood],
    decision: &DecisionConfig,
) -> Vec<CandidateScore> {
    if hunger.level < hunger.seek_threshold {
        return Vec::new();
    }

    let hunger_pressure = if hunger.max <= 0.0 {
        0.0
    } else {
        (hunger.level / hunger.max).clamp(0.0, 1.0)
    };

    let mut candidates = perceived
        .iter()
        .map(|food| {
            let nutrition_value = food.nutrition * decision.nutrition_weight;
            let distance_penalty = food.distance * decision.distance_weight;
            let hunger_boost = hunger_pressure * decision.hunger_weight;
            let score = hunger_boost + nutrition_value - distance_penalty;

            CandidateScore {
                id: food.id.clone(),
                distance: food.distance,
                nutrition: food.nutrition,
                score,
                score_parts: ScoreBreakdown {
                    hunger_pressure: hunger_boost,
                    nutrition_value,
                    distance_penalty,
                },
            }
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.distance.total_cmp(&right.distance))
            .then_with(|| left.id.cmp(&right.id))
    });
    candidates
}

pub fn step_agent(input: AgentStepInput) -> AgentStepResult {
    let perceived = perceive_foods(input.agent_position, input.perception_radius, &input.foods);
    let candidates = score_candidates(input.hunger, &perceived, &input.decision);

    let chosen_target = choose_target(&input.current_target, &perceived, &candidates);
    let mut mind = AgentMind {
        perceived,
        candidates,
        selected_target: chosen_target.clone(),
        intention: Intention::Idle,
    };

    let Some(target_id) = chosen_target else {
        return AgentStepResult {
            mind,
            move_target: None,
            consumed_food: None,
            hunger_after_consumption: None,
        };
    };

    let target = input
        .foods
        .iter()
        .find(|food| food.available && food.id == target_id);

    let Some(target) = target else {
        mind.selected_target = None;
        return AgentStepResult {
            mind,
            move_target: None,
            consumed_food: None,
            hunger_after_consumption: None,
        };
    };

    let delta = target.position - input.agent_position;
    let horizontal = Vec3::new(delta.x, 0.0, delta.z);
    let distance = horizontal.length();

    if distance <= input.consume_distance {
        mind.intention = Intention::ConsumeFood {
            food_id: target.id.clone(),
        };
        let hunger_after = (input.hunger.level - target.nutrition).clamp(0.0, input.hunger.max);
        return AgentStepResult {
            mind,
            move_target: None,
            consumed_food: Some(target.id.clone()),
            hunger_after_consumption: Some(hunger_after),
        };
    }

    mind.intention = Intention::SeekFood {
        food_id: target.id.clone(),
    };
    AgentStepResult {
        mind,
        move_target: Some(target.position),
        consumed_food: None,
        hunger_after_consumption: None,
    }
}

fn choose_target(
    current_target: &Option<String>,
    perceived: &[PerceivedFood],
    candidates: &[CandidateScore],
) -> Option<String> {
    if let Some(current) = current_target
        && perceived.iter().any(|food| &food.id == current)
        && candidates.iter().any(|candidate| &candidate.id == current)
    {
        return Some(current.clone());
    }

    candidates.first().map(|candidate| candidate.id.clone())
}

pub fn food_world_state(food: &FoodConfig) -> FoodWorldState {
    FoodWorldState {
        id: food.id.clone(),
        position: food.position.as_vec3(),
        nutrition: food.nutrition,
        available: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::config::{DecisionConfig, Vec3Config};

    fn decision() -> DecisionConfig {
        DecisionConfig {
            hunger_weight: 1.0,
            nutrition_weight: 1.0,
            distance_weight: 0.5,
        }
    }

    fn hunger(level: f32) -> HungerState {
        HungerState {
            level,
            max: 100.0,
            seek_threshold: 60.0,
        }
    }

    fn food(id: &str, x: f32, z: f32, nutrition: f32) -> FoodWorldState {
        FoodWorldState {
            id: id.to_string(),
            position: Vec3::new(x, 0.25, z),
            nutrition,
            available: true,
        }
    }

    #[test]
    fn hunger_growth_clamps_to_maximum() {
        assert_eq!(increase_hunger(95.0, 10.0, 100.0, 1.0), 100.0);
    }

    #[test]
    fn perception_filters_outside_radius() {
        let foods = vec![food("near", 2.0, 0.0, 10.0), food("far", 20.0, 0.0, 50.0)];
        let perceived = perceive_foods(Vec3::ZERO, 5.0, &foods);

        assert_eq!(perceived.len(), 1);
        assert_eq!(perceived[0].id, "near");
    }

    #[test]
    fn candidate_scoring_can_prefer_nutrition_over_distance() {
        let perceived = vec![
            PerceivedFood {
                id: "close".to_string(),
                position: Vec3::new(2.0, 0.25, 0.0),
                distance: 2.0,
                nutrition: 5.0,
            },
            PerceivedFood {
                id: "rich".to_string(),
                position: Vec3::new(4.0, 0.25, 0.0),
                distance: 4.0,
                nutrition: 12.0,
            },
        ];

        let candidates = score_candidates(hunger(80.0), &perceived, &decision());
        assert_eq!(candidates[0].id, "rich");
    }

    #[test]
    fn consumption_occurs_inside_consume_distance() {
        let result = step_agent(AgentStepInput {
            agent_position: Vec3::new(0.0, 0.8, 0.0),
            hunger: hunger(75.0),
            perception_radius: 10.0,
            consume_distance: 1.0,
            current_target: None,
            foods: vec![food("apple", 0.5, 0.0, 20.0)],
            decision: decision(),
        });

        assert_eq!(result.consumed_food.as_deref(), Some("apple"));
        assert_eq!(result.mind.selected_target.as_deref(), Some("apple"));
    }

    #[test]
    fn invalid_target_replans_to_another_candidate() {
        let result = step_agent(AgentStepInput {
            agent_position: Vec3::ZERO,
            hunger: hunger(80.0),
            perception_radius: 20.0,
            consume_distance: 0.5,
            current_target: Some("gone".to_string()),
            foods: vec![food("backup", 3.0, 0.0, 15.0)],
            decision: decision(),
        });

        assert_eq!(result.mind.selected_target.as_deref(), Some("backup"));
        assert!(matches!(
            result.mind.intention,
            Intention::SeekFood { ref food_id } if food_id == "backup"
        ));
    }

    #[test]
    fn below_threshold_agent_stays_idle() {
        let result = step_agent(AgentStepInput {
            agent_position: Vec3::ZERO,
            hunger: hunger(20.0),
            perception_radius: 20.0,
            consume_distance: 0.5,
            current_target: None,
            foods: vec![food("apple", 3.0, 0.0, 15.0)],
            decision: decision(),
        });

        assert!(result.mind.candidates.is_empty());
        assert_eq!(result.mind.selected_target, None);
        assert_eq!(result.mind.intention, Intention::Idle);
    }

    #[test]
    fn food_world_state_uses_semantic_config() {
        let world = food_world_state(&FoodConfig {
            id: "berry".to_string(),
            position: Vec3Config {
                x: 1.0,
                y: 0.25,
                z: 2.0,
            },
            nutrition: 9.0,
        });

        assert_eq!(world.id, "berry");
        assert_eq!(world.position, Vec3::new(1.0, 0.25, 2.0));
    }
}
