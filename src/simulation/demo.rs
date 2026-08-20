use bevy::prelude::*;
use bevy::text::{FontSize, FontSource};

use super::components::{
    Agent, AgentBodyState, AgentBrain, AgentDebugText, AgentMotion, Food, Hunger, Perception,
};
use super::config::{AgentDemoConfig, parse_color_or};

pub fn setup_demo(
    mut commands: Commands,
    config: Res<AgentDemoConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mesh = meshes.add(
        Plane3d::default()
            .mesh()
            .size(config.world.floor_size, config.world.floor_size),
    );
    let agent_mesh = meshes.add(Capsule3d::new(0.35, 1.0));
    let food_mesh = meshes.add(Sphere::new(0.25));

    let floor_material = materials.add(parse_color_or(
        &config.world.floor_color,
        Color::srgb(0.18, 0.20, 0.22),
    ));
    let agent_material = materials.add(Color::srgb(0.25, 0.55, 0.95));
    let food_material = materials.add(Color::srgb(0.30, 0.90, 0.35));

    commands.spawn((Mesh3d(floor_mesh), MeshMaterial3d(floor_material)));

    commands.spawn((
        Mesh3d(agent_mesh),
        MeshMaterial3d(agent_material),
        Transform::from_translation(config.agent.position.as_vec3()),
        Agent,
        Hunger {
            level: config.agent.hunger.initial,
            max: config.agent.hunger.max,
            rate_per_second: config.agent.hunger.rate_per_second,
            seek_threshold: config.agent.hunger.seek_threshold,
        },
        AgentMotion {
            speed: config.agent.motion.speed,
            consume_distance: config.agent.motion.consume_distance,
        },
        Perception {
            radius: config.agent.perception_radius,
        },
        AgentBrain::default(),
        AgentBodyState::default(),
    ));

    for food in &config.foods {
        commands.spawn((
            Mesh3d(food_mesh.clone()),
            MeshMaterial3d(food_material.clone()),
            Transform::from_translation(food.position.as_vec3()),
            Food {
                id: food.id.clone(),
                nutrition: food.nutrition,
            },
            Name::new(format!("food_{}", food.id)),
        ));
    }

    commands.spawn((
        PointLight {
            intensity: 4_000_000.0,
            range: 30.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(config.camera.position.as_vec3())
            .looking_at(config.camera.look_at.as_vec3(), Vec3::Y),
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            top: Val::Px(16.0),
            max_width: Val::Px(460.0),
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.03, 0.03, 0.03, 0.82)),
        Text::new(String::new()),
        TextFont {
            font: FontSource::default(),
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::WHITE),
        AgentDebugText,
    ));
}
