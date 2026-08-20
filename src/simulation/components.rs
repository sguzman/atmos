use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Agent;

#[derive(Component, Debug)]
pub struct Food {
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
pub struct FoodTarget(pub Option<Entity>);
