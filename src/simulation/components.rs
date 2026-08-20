use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Agent;

#[derive(Component, Debug)]
pub struct Food {
    pub id: String,
    pub nutrition: f32,
}

#[derive(Component, Debug)]
pub struct Hunger {
    pub level: f32,
    pub max: f32,
    pub rate_per_second: f32,
    pub seek_threshold: f32,
}

#[derive(Component, Debug)]
pub struct AgentMotion {
    pub speed: f32,
    pub consume_distance: f32,
}

#[derive(Component, Debug)]
pub struct Perception {
    pub radius: f32,
}

#[derive(Component, Debug, Default)]
pub struct AgentBodyState {
    pub look_target: Option<Vec3>,
    pub desired_velocity: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct AgentDebugText;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SimulationClock {
    pub elapsed: f32,
}

impl Default for SimulationClock {
    fn default() -> Self {
        Self { elapsed: 0.0 }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Intention {
    #[default]
    Idle,
    SeekFood {
        food_id: String,
    },
    ConsumeFood {
        food_id: String,
    },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AgentMind {
    pub perceived: Vec<PerceivedFood>,
    pub candidates: Vec<CandidateScore>,
    pub selected_target: Option<String>,
    pub intention: Intention,
}

#[derive(Component, Debug, Default)]
pub struct AgentBrain {
    pub mind: AgentMind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerceivedFood {
    pub id: String,
    pub position: Vec3,
    pub distance: f32,
    pub nutrition: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateScore {
    pub id: String,
    pub distance: f32,
    pub nutrition: f32,
    pub score: f32,
    pub score_parts: ScoreBreakdown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreBreakdown {
    pub hunger_pressure: f32,
    pub nutrition_value: f32,
    pub distance_penalty: f32,
}
