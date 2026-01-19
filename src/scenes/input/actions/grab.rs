use bevy::{
    prelude::{
        Alpha, AlphaMode, Assets,
        ChildOf, Commands, Component,
        Entity, GlobalTransform,
        InheritedVisibility, Query,
        Res, ResMut, StandardMaterial,
        Transform, Vec3,
        ViewVisibility, Visibility,
        With, Without, default,
    },
    render::render_resource::Face,
};
use bevy_rapier3d::prelude::{
    GravityScale, QueryFilter,
    ReadRapierContext, RigidBody,
    Sensor, Velocity,
};

use super::super::types::{
    ActionStates, GrabHover, GrabState,
    PlayerBody, SceneCamera,
    SceneGrabConfig,
};

#[derive(Component)]
pub(crate) struct GrabOutline;

#[derive(Component)]
pub(crate) struct GrabbedBody {
    original_body: RigidBody,
    original_gravity: f32,
    original_sensor: bool,
}

pub fn update_grab_hover(
    config: Option<
        Res<SceneGrabConfig>,
    >,
    rapier_context: ReadRapierContext,
    cameras: Query<
        &GlobalTransform,
        With<SceneCamera>,
    >,
    player: Query<
        Entity,
        With<PlayerBody>,
    >,
    parents: Query<&ChildOf>,
    bodies: Query<&RigidBody>,
    meshes: Query<
        &bevy::prelude::Mesh3d,
    >,
    children: Query<
        &bevy::prelude::Children,
    >,
    outlines: Query<
        Entity,
        With<GrabOutline>,
    >,
    mut materials: ResMut<
        Assets<StandardMaterial>,
    >,
    mut commands: Commands,
    hover: Option<ResMut<GrabHover>>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(mut hover) = hover else {
        return;
    };
    let Ok(camera) = cameras.single()
    else {
        return;
    };
    let Ok(context) =
        rapier_context.single()
    else {
        return;
    };

    let player_body =
        player.single().ok();
    let filter = QueryFilter {
        exclude_rigid_body: player_body,
        ..Default::default()
    };
    let origin = camera.translation();
    let dir =
        camera.forward().as_vec3();
    let max_toi =
        config.action.range.max(0.0);
    let hit = context.cast_ray(
        origin, dir, max_toi, true,
        filter,
    );

    let mut target = None;
    if let Some((entity, _)) = hit {
        let candidate = if meshes
            .get(entity)
            .is_ok()
        {
            Some(entity)
        } else if let Ok(parent) =
            parents.get(entity)
        {
            let parent_entity =
                parent.parent();
            if meshes
                .get(parent_entity)
                .is_ok()
            {
                Some(parent_entity)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(entity) = candidate
        {
            if let Ok(body) =
                bodies.get(entity)
            {
                if !matches!(
                    body,
                    RigidBody::Fixed
                ) {
                    target =
                        Some(entity);
                }
            }
        }
    }

    if target == hover.entity {
        return;
    }

    if let Some(prev) = hover.entity {
        if let Ok(children) =
            children.get(prev)
        {
            for child in children.iter()
            {
                if outlines
                    .get(*child)
                    .is_ok()
                {
                    commands
                        .entity(*child)
                        .queue_silenced(bevy::ecs::system::entity_command::despawn());
                }
            }
        }
    }

    if let Some(target) = target {
        if let Ok(mesh) =
            meshes.get(target)
        {
            let mut color =
                config.outline_color;
            let opacity = config
                .action
                .outline
                .opacity
                .clamp(0.0, 1.0);
            color.set_alpha(opacity);

            let outline_material = materials.add(StandardMaterial {
                base_color: color,
                emissive: color.to_linear(),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                cull_mode: Some(Face::Front),
                ..default()
            });

            let thickness = config
                .action
                .outline
                .thickness
                .max(0.0);
            let scale = 1.0 + thickness;
            commands.entity(target).with_children(|parent| {
                parent.spawn((
                    bevy::prelude::Mesh3d(mesh.0.clone()),
                    bevy::prelude::MeshMaterial3d(outline_material),
                    Transform::from_scale(Vec3::splat(scale)),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    GrabOutline,
                ));
            });
        }
    }

    hover.entity = target;
}

pub fn update_grab_hold(
    config: Option<
        Res<SceneGrabConfig>,
    >,
    state: Option<Res<GrabState>>,
    cameras: Query<
        &GlobalTransform,
        With<SceneCamera>,
    >,
    mut bodies: Query<
        &mut Transform,
        With<GrabbedBody>,
    >,
) {
    let Some(config) = config else {
        return;
    };
    let Some(state) = state else {
        return;
    };
    let Some(held) = state.held else {
        return;
    };
    let Ok(camera) = cameras.single()
    else {
        return;
    };
    let Ok(mut transform) =
        bodies.get_mut(held)
    else {
        return;
    };

    let offset = Vec3::new(
        config.action.hold_offset.x,
        config.action.hold_offset.y,
        config.action.hold_offset.z,
    );
    let camera_transform =
        camera.compute_transform();
    let target = camera.translation()
        + camera.forward()
            * config
                .action
                .hold_distance
        + camera_transform.rotation
            * offset;
    transform.translation = target;
}

pub fn apply_grab_action(
    config: Option<
        Res<SceneGrabConfig>,
    >,
    states: Option<Res<ActionStates>>,
    state: Option<ResMut<GrabState>>,
    hover: Option<Res<GrabHover>>,
    cameras: Query<
        &GlobalTransform,
        With<SceneCamera>,
    >,
    grabbed: Query<&GrabbedBody>,
    player_velocity: Query<
        &Velocity,
        With<PlayerBody>,
    >,
    mut bodies: Query<
        (
            Entity,
            &mut RigidBody,
            Option<&mut GravityScale>,
            Option<&mut Velocity>,
            Option<&Sensor>,
        ),
        Without<PlayerBody>,
    >,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };
    let Some(states) = states else {
        return;
    };
    let Some(mut state) = state else {
        return;
    };
    let Some(hover) = hover else {
        return;
    };
    if !states
        .get(&config.id)
        .just_pressed
    {
        return;
    }

    let Ok(camera) = cameras.single()
    else {
        return;
    };
    let player_linvel = player_velocity
        .iter()
        .next()
        .map(|vel| vel.linvel)
        .unwrap_or(Vec3::ZERO);

    if let Some(held) = state.held {
        if let Ok((
            entity,
            mut body,
            mut gravity,
            velocity,
            _sensor,
        )) = bodies.get_mut(held)
        {
            if let Ok(grabbed) =
                grabbed.get(entity)
            {
                *body = grabbed
                    .original_body;
                let gravity_value = grabbed.original_gravity;
                match gravity.as_mut() {
                    Some(gravity) => gravity.0 = gravity_value,
                    None => {
                        commands.entity(entity).insert(GravityScale(gravity_value));
                    }
                }
                commands.entity(entity).remove::<GrabbedBody>();
                if !config
                    .action
                    .collision
                    && !grabbed
                        .original_sensor
                {
                    commands.entity(entity).remove::<Sensor>();
                }
            } else {
                *body =
                    RigidBody::Dynamic;
                match gravity.as_mut() {
                    Some(gravity) => {
                        gravity.0 = 1.0
                    }
                    None => {
                        commands.entity(entity).insert(GravityScale(1.0));
                    }
                }
            }

            let throw_velocity = camera
                .forward()
                .as_vec3()
                * config
                    .action
                    .throw_speed
                    .max(0.0)
                + player_linvel;
            if let Some(mut velocity) =
                velocity
            {
                velocity.linvel =
                    throw_velocity;
                velocity.angvel =
                    Vec3::ZERO;
            } else {
                commands.entity(entity).insert(Velocity {
                    linvel: throw_velocity,
                    angvel: Vec3::ZERO,
                });
            }
        }
        state.held = None;
        return;
    }

    let Some(target) = hover.entity
    else {
        return;
    };

    if let Ok((
        entity,
        mut body,
        mut gravity,
        velocity,
        sensor,
    )) = bodies.get_mut(target)
    {
        if matches!(
            *body,
            RigidBody::Fixed
        ) {
            return;
        }
        let original_gravity = gravity
            .as_ref()
            .map(|g| g.0)
            .unwrap_or(1.0);
        let original_sensor =
            sensor.is_some();
        commands.entity(entity).insert(
            GrabbedBody {
                original_body: *body,
                original_gravity,
                original_sensor,
            },
        );
        *body = RigidBody::KinematicPositionBased;
        match gravity.as_mut() {
            Some(gravity) => {
                gravity.0 = 0.0
            }
            None => {
                commands
                    .entity(entity)
                    .insert(
                        GravityScale(
                            0.0,
                        ),
                    );
            }
        }
        if !config.action.collision
            && sensor.is_none()
        {
            commands
                .entity(entity)
                .insert(Sensor);
        }
        if let Some(mut velocity) =
            velocity
        {
            velocity.linvel =
                Vec3::ZERO;
            velocity.angvel =
                Vec3::ZERO;
        } else {
            commands
                .entity(entity)
                .insert(
                    Velocity::default(),
                );
        }
        state.held = Some(entity);
    }
}
