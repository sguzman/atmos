use bevy::input::mouse::{
    MouseButton, MouseMotion,
    MouseWheel,
};
use bevy::{
    asset::RenderAssetUsages,
    log::warn,
    math::Vec3,
    prelude::{
        AlphaMode, Assets, ButtonInput,
        ChildOf,
        Color, Commands, Component,
        Entity, GlobalTransform,
        Handle, InheritedVisibility,
        Mesh, Mesh3d, MeshMaterial3d,
        MessageReader, Name, Quat,
        Query, Res, ResMut, Resource,
        StandardMaterial, Transform,
        ViewVisibility, Visibility,
        With, default,
    },
    render::render_resource::PrimitiveTopology,
};
use bevy_mesh::Indices;
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider,
    Friction, QueryFilter,
    ReadRapierContext, Restitution,
    RigidBody,
};

use crate::scenes::{
    MeshCacheSettings,
    bounds::DespawnOutsideBounds,
    config::{
        CutActivationMode, PhysicsConfig,
        ShapeConfig, parse_color,
    },
    input::{
        ActionStates, CutHover,
        PlayerBody, SceneCamera,
        SceneCutAxisConfig, SceneCutConfig,
    },
    mesh_cache::{
        cache_mesh, load_cached_mesh,
    },
    spawn::SceneEntityTag,
};

#[derive(Component)]
pub struct CutPlanePreview;

#[derive(Component, Clone)]
pub struct CuttableShape {
    pub shape: ShapeConfig,
    pub physics: Option<PhysicsConfig>,
    pub material:
        Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
pub struct CutState {
    pub preview: Option<Entity>,
    pub hovered: Option<Entity>,
    pub angle_index: i32,
    pub axis: CutRotationAxis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutRotationAxis {
    Yaw,
    Pitch,
    Roll,
}

impl Default for CutRotationAxis {
    fn default() -> Self {
        CutRotationAxis::Yaw
    }
}

impl CutState {
    fn step_count(
        &self,
        step_degrees: f32,
    ) -> i32 {
        let step = step_degrees.max(1.0_f32);
        (360.0_f32 / step)
            .round()
            .max(1.0) as i32
    }

    fn angle_radians(
        &self,
        step_degrees: f32,
    ) -> f32 {
        let angle_deg =
            (self.angle_index as f32) * step_degrees;
        angle_deg.to_radians()
    }

    fn set_angle_by_step(
        &mut self,
        steps: i32,
        total_steps: i32,
    ) {
        let wrapped = ((steps
            % total_steps)
            + total_steps)
            % total_steps;
        self.angle_index = wrapped;
    }
}

fn cut_axis_rotations(
    axis: CutRotationAxis,
    angle: f32,
) -> (Quat, Quat) {
    let axis_rotation = match axis {
        CutRotationAxis::Yaw => {
            Quat::from_rotation_y(angle)
        }
        CutRotationAxis::Pitch => {
            Quat::from_rotation_x(angle)
        }
        CutRotationAxis::Roll => {
            Quat::from_rotation_z(angle)
        }
    };
    let base_rotation = match axis {
        // The preview mesh is a thin slab with normal along +Z.
        // Roll mode would not change that normal, so pre-rotate
        // the slab so its normal starts along +Y.
        CutRotationAxis::Roll => {
            Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )
        }
        _ => Quat::IDENTITY,
    };
    (axis_rotation, base_rotation)
}

fn cut_plane_normal_local(
    axis: CutRotationAxis,
    angle: f32,
) -> Vec3 {
    let (axis_rotation, base_rotation) =
        cut_axis_rotations(axis, angle);
    let local =
        (axis_rotation * base_rotation)
            * Vec3::Z;
    if local.length_squared() < 1e-6 {
        Vec3::Z
    } else {
        local.normalize()
    }
}

#[derive(Resource, Default)]
pub struct CutPreviewAssets {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<
        Handle<StandardMaterial>,
    >,
}

#[derive(Resource, Default)]
pub struct CutActivationState {
    pub active: bool,
}

fn ensure_preview_assets(
    config: &SceneCutConfig,
    assets: &mut CutPreviewAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<
        StandardMaterial,
    >,
) -> (
    Handle<Mesh>,
    Handle<StandardMaterial>,
) {
    if assets.mesh.is_none() {
        let mesh = create_preview_plane(
            meshes,
        );
        assets.mesh = Some(mesh);
    }
    if assets.material.is_none() {
        let rgb = parse_color(
            &config
                .action
                .preview_color,
        )
        .unwrap_or([255, 0, 255]);
        let alpha = (config
            .action
            .preview_opacity
            * 255.0_f32)
            .clamp(0.0_f32, 255.0_f32)
            as u8;
        let color = Color::srgba_u8(
            rgb[0], rgb[1], rgb[2],
            alpha,
        );
        let emissive = color.to_linear()
            * config
                .action
                .preview_emissive
                .max(0.0);
        let mat = materials.add(
            StandardMaterial {
                base_color: color,
                emissive,
                unlit: true,
                alpha_mode:
                    AlphaMode::Blend,
                // Avoid z-fighting with the grab outline / hover highlight.
                depth_bias: 1.0,
                cull_mode: None,
                ..default()
            },
        );
        assets.material = Some(mat);
    }
    (
        assets
            .mesh
            .as_ref()
            .unwrap()
            .clone(),
        assets
            .material
            .as_ref()
            .unwrap()
            .clone(),
    )
}

fn create_preview_plane(
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    meshes.add(Mesh::from(
        bevy::math::primitives::Cuboid::new(
            1.0, 1.0, 1.0,
        ),
    ))
}

pub fn update_cut_hover(
    cut_config: Option<
        Res<SceneCutConfig>,
    >,
    states: Option<Res<ActionStates>>,
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
    mut cut_hover: ResMut<CutHover>,
    mut activation_state: ResMut<CutActivationState>,
    cuttables: Query<&CuttableShape>,
) {
    let Some(config) = cut_config
    else {
        return;
    };
    let Some(states) = states else {
        cut_hover.entity = None;
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

    let action_state = states.get(&config.id);
    let mut action_active =
        action_state.pressed;
    if config.action.mode
        == CutActivationMode::Toggle
    {
        if action_state.just_pressed {
            activation_state.active =
                !activation_state
                    .active;
        }
        action_active =
            activation_state.active;
    }
    if !action_active {
        cut_hover.entity = None;
        return;
    }

    let player_body =
        player.single().ok();
    let filter = QueryFilter {
        exclude_rigid_body: player_body,
        ..Default::default()
    };

    let origin = camera.translation();
    let dir =
        camera.forward().as_vec3();
    let max_toi = 8.0;

    let mut target = None;
    if let Some((entity, _)) = context
        .cast_ray(
            origin, dir, max_toi, true,
            filter,
        )
    {
        if cuttables.get(entity).is_ok()
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
        } else if let Ok(parent) =
            parents.get(entity)
        {
            if let Ok(body) = bodies
                .get(parent.parent())
            {
                if !matches!(
                    body,
                    RigidBody::Fixed
                ) && cuttables
                    .get(
                        parent.parent(),
                    )
                    .is_ok()
                {
                    target = Some(
                        parent.parent(),
                    );
                }
            }
        }
    }

    cut_hover.entity = target;
}

pub fn update_cut_preview(
    cut_config: Option<
        Res<SceneCutConfig>,
    >,
    cut_axis_config: Option<
        Res<SceneCutAxisConfig>,
    >,
    states: Option<Res<ActionStates>>,
    cut_hover: Option<Res<CutHover>>,
    mut cut_state: ResMut<CutState>,
    mut preview_assets: ResMut<
        CutPreviewAssets,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<StandardMaterial>,
    >,
    mut commands: Commands,
    mut motion_events: MessageReader<
        MouseMotion,
    >,
    mut wheel_events: MessageReader<
        MouseWheel,
    >,
    mouse_input: Res<
        ButtonInput<MouseButton>,
    >,
    cut_activation: Res<CutActivationState>,
    transforms: Query<
        &GlobalTransform,
        With<CuttableShape>,
    >,
    cuttables: Query<&CuttableShape>,
    mesh_cache: Res<MeshCacheSettings>,
) {
    let Some(config) = cut_config
    else {
        return;
    };
    let Some(states) = states else {
        clear_cut_preview(
            &mut commands,
            &mut cut_state,
        );
        return;
    };

    let action_state =
        states.get(&config.id);
    let action_active = if config
        .action
        .mode
        == CutActivationMode::Toggle
    {
        cut_activation.active
    } else {
        action_state.pressed
    };
    if !action_active {
        clear_cut_preview(
            &mut commands,
            &mut cut_state,
        );
        return;
    }

    let Some(cut_hover) = cut_hover
    else {
        return;
    };
    let Some(target) = cut_hover.entity
    else {
        clear_cut_preview(
            &mut commands,
            &mut cut_state,
        );
        return;
    };

    if cut_state.hovered != Some(target)
    {
        cut_state.hovered =
            Some(target);
        cut_state.angle_index = 0;
    }

    let effective_step_degrees =
        cut_axis_config
            .as_ref()
            .and_then(|axis| {
                axis.action
                    .angle_step_degrees_override
            })
            .unwrap_or(config.action.angle_step_degrees);

    if let Some(axis_config) =
        cut_axis_config.as_ref()
    {
        let axis_state =
            states.get(&axis_config.id);
        if axis_state.just_pressed {
            cut_state.axis = match cut_state.axis {
                CutRotationAxis::Yaw => {
                    CutRotationAxis::Pitch
                }
                CutRotationAxis::Pitch => {
                    CutRotationAxis::Roll
                }
                CutRotationAxis::Roll => {
                    CutRotationAxis::Yaw
                }
            };
            if axis_config
                .action
                .reset_angle_on_switch
            {
                cut_state.angle_index = 0;
            }
        }
    }

    apply_mouse_rotation(
        &mut cut_state,
        &config,
        effective_step_degrees,
        &mut motion_events,
        &mut wheel_events,
    );

    let (mesh_handle, material_handle) =
        ensure_preview_assets(
            &config,
            &mut preview_assets,
            &mut meshes,
            &mut materials,
        );

    if cut_state.preview.is_none() {
        let preview = commands
            .spawn((
                Name::new("cut_preview"),
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                CutPlanePreview,
            ))
            .id();
        cut_state.preview =
            Some(preview);
    }

    if let Some(preview_entity) =
        cut_state.preview
    {
        if let Ok(target_transform) =
            transforms.get(target)
        {
            let angle = cut_state
                .angle_radians(
                    effective_step_degrees,
                );
            let (axis_rotation, base_rotation) =
                cut_axis_rotations(
                    cut_state.axis,
                    angle,
                );
            let rotation = target_transform
                .rotation()
                * (axis_rotation
                    * base_rotation);
            let translation =
                target_transform
                    .translation();
            let scale = Vec3::new(
                config
                    .action
                    .preview_size
                    .max(0.1),
                config
                    .action
                    .preview_size
                    .max(0.1),
                config
                    .action
                    .preview_thickness
                    .max(0.001),
            );
            commands
                .entity(preview_entity)
                .insert(Transform {
                    translation,
                    rotation,
                    scale,
                });
        }
    }

    if mouse_input
        .just_pressed(config.confirm_button)
    {
        if let Ok(cuttable) =
            cuttables.get(target)
        {
            if let Ok(transform) =
                transforms.get(target)
            {
                if perform_cut(
                    &mut commands,
                    &mesh_cache,
                    &mut meshes,
                    target,
                    cuttable,
                    transform,
                    cut_state.axis,
                    cut_state
                        .angle_index,
                    cut_state
                        .angle_radians(
                            effective_step_degrees,
                        ),
                    effective_step_degrees,
                    &config,
                ) {
                    clear_cut_preview(
                        &mut commands,
                        &mut cut_state,
                    );
                }
            }
        }
    }
}

fn apply_mouse_rotation(
    cut_state: &mut CutState,
    config: &SceneCutConfig,
    step_degrees: f32,
    motion_events: &mut MessageReader<
        MouseMotion,
    >,
    wheel_events: &mut MessageReader<
        MouseWheel,
    >,
) {
    let steps_per_pixel = config
        .action
        .rotation_sensitivity;
    if steps_per_pixel <= 0.0 {
        return;
    }
    let step_count =
        cut_state.step_count(step_degrees);
    let mut delta_steps = 0;
    for motion in motion_events.read() {
        delta_steps += (motion.delta.x
            * steps_per_pixel)
            .round()
            as i32;
    }
    for wheel in wheel_events.read() {
        delta_steps += (wheel.y
            * config
                .action
                .wheel_rotation_sensitivity)
            .round()
            as i32;
    }
    if delta_steps != 0 {
        let new_index = cut_state
            .angle_index
            + delta_steps;
        cut_state.set_angle_by_step(
            new_index, step_count,
        );
    }
}

fn clear_cut_preview(
    commands: &mut Commands,
    cut_state: &mut CutState,
) {
    if let Some(preview) =
        cut_state.preview.take()
    {
        commands
            .entity(preview)
            .despawn();
    }
    cut_state.hovered = None;
    cut_state.angle_index = 0;
    cut_state.axis = CutRotationAxis::Yaw;
}

pub fn cleanup_cut_state(
    mut commands: Commands,
    mut cut_state: ResMut<CutState>,
    mut activation_state:
        ResMut<CutActivationState>,
) {
    clear_cut_preview(
        &mut commands,
        &mut cut_state,
    );
    activation_state.active =
        false;
}

fn perform_cut(
    commands: &mut Commands,
    settings: &MeshCacheSettings,
    meshes: &mut Assets<Mesh>,
    target: Entity,
    cuttable: &CuttableShape,
    transform: &GlobalTransform,
    axis: CutRotationAxis,
    angle_index: i32,
    angle: f32,
    step_degrees: f32,
    _config: &SceneCutConfig,
) -> bool {
    const CUT_CACHE_VERSION: &str = "v6";
    let Some(dimensions) = cuttable
        .shape
        .dimensions
        .as_ref()
    else {
        warn!(
            "Cuttable shape lacks dimensions."
        );
        return false;
    };

    let half_extents = Vec3::new(
        dimensions.width * 0.5,
        dimensions.height * 0.5,
        dimensions.depth * 0.5,
    );

    let vertices =
        cube_vertices(half_extents);
    let cube_tris =
        build_cube_triangles(&vertices);

    let local_normal =
        cut_plane_normal_local(axis, angle);
    if local_normal.length_squared() < 1e-6 {
        warn!(
            "Plane normal is degenerate; cannot cut."
        );
        return false;
    }
    let plane_normal = local_normal;

    let mut positive_tris =
        clip_triangles(
            &cube_tris,
            plane_normal,
            true,
        );
    let mut negative_tris =
        clip_triangles(
            &cube_tris,
            plane_normal,
            false,
        );

    let intersection =
        plane_intersection_polygon(
            &vertices,
            plane_normal,
        );
    if intersection.len() >= 3 {
        positive_tris.extend(
            build_cap_triangles(
                &intersection,
                plane_normal,
                false,
            ),
        );
        negative_tris.extend(
            build_cap_triangles(
                &intersection,
                plane_normal,
                true,
            ),
        );
    }

    if positive_tris.is_empty()
        || negative_tris.is_empty()
    {
        warn!(
            "Plane did not produce two valid halves."
        );
        return false;
    }

    let base_key = format!(
        "cut_{CUT_CACHE_VERSION}_cube_w{}_h{}_d{}_axis{}_step{}",
        format_key(dimensions.width),
        format_key(dimensions.height),
        format_key(dimensions.depth),
        match axis {
            CutRotationAxis::Yaw => "yaw",
            CutRotationAxis::Pitch => "pitch",
            CutRotationAxis::Roll => "roll",
        },
        format_key(step_degrees),
    );
    let angle_key = format_key(
        (angle_index as f32)
            * step_degrees,
    );
    let pos_key = format!(
        "{base_key}_a{angle_key}_pos"
    );
    let neg_key = format!(
        "{base_key}_a{angle_key}_neg"
    );

    let max_dim = half_extents
        .x
        .max(half_extents.y)
        .max(half_extents.z)
        .max(1e-4);
    let (
        pos_handle,
        pos_positions,
        pos_indices,
    ) = ensure_cached_mesh(
        settings,
        meshes,
        &positive_tris,
        &pos_key,
        max_dim,
        -plane_normal,
    );
    let (
        neg_handle,
        neg_positions,
        neg_indices,
    ) = ensure_cached_mesh(
        settings,
        meshes,
        &negative_tris,
        &neg_key,
        max_dim,
        plane_normal,
    );

    let pos_collider = Collider::trimesh(
        positions_to_points(&pos_positions),
        pos_indices.clone(),
    )
    .expect("Failed to build positive cut collider");
    let neg_collider = Collider::trimesh(
        positions_to_points(&neg_positions),
        neg_indices.clone(),
    )
    .expect("Failed to build negative cut collider");

    let trans = Transform::from_matrix(
        transform.to_matrix(),
    );
    spawn_half_entity(
        commands,
        pos_handle,
        pos_collider,
        cuttable,
        &trans,
    );
    spawn_half_entity(
        commands,
        neg_handle,
        neg_collider,
        cuttable,
        &trans,
    );

    commands.entity(target).despawn();
    true
}

const CUBE_TRIANGLE_INDICES:
    &[[usize; 3]] = &[
    [0, 1, 2],
    [2, 3, 0],
    [4, 5, 1],
    [1, 0, 4],
    [2, 6, 5],
    [5, 1, 2],
    [3, 2, 6],
    [6, 7, 3],
    [4, 0, 3],
    [3, 7, 4],
    [5, 6, 7],
    [7, 4, 5],
];

const CUBE_EDGE_PAIRS: &[(
    usize,
    usize,
)] = &[
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

fn cube_vertices(
    half: Vec3,
) -> [Vec3; 8] {
    [
        Vec3::new(
            -half.x, -half.y, -half.z,
        ),
        Vec3::new(
            half.x, -half.y, -half.z,
        ),
        Vec3::new(
            half.x, half.y, -half.z,
        ),
        Vec3::new(
            -half.x, half.y, -half.z,
        ),
        Vec3::new(
            -half.x, -half.y, half.z,
        ),
        Vec3::new(
            half.x, -half.y, half.z,
        ),
        Vec3::new(
            half.x, half.y, half.z,
        ),
        Vec3::new(
            -half.x, half.y, half.z,
        ),
    ]
}

fn build_cube_triangles(
    vertices: &[Vec3; 8],
) -> Vec<[Vec3; 3]> {
    CUBE_TRIANGLE_INDICES
        .iter()
        .map(|[a, b, c]| {
            [
                vertices[*a],
                vertices[*b],
                vertices[*c],
            ]
        })
        .collect()
}

fn clip_triangles(
    triangles: &[[Vec3; 3]],
    plane_normal: Vec3,
    keep_positive: bool,
) -> Vec<[Vec3; 3]> {
    let mut output = Vec::new();
    for tri in triangles {
        let clipped = clip_polygon(
            tri,
            plane_normal,
            keep_positive,
        );
        output.extend(
            polygon_to_triangles(
                &clipped,
            ),
        );
    }
    output
}

fn clip_polygon(
    polygon: &[Vec3; 3],
    plane_normal: Vec3,
    keep_positive: bool,
) -> Vec<Vec3> {
    let mut output = Vec::new();
    for i in 0..polygon.len() {
        let current = polygon[i];
        let next = polygon
            [(i + 1) % polygon.len()];
        let current_dot =
            plane_normal.dot(current);
        let next_dot =
            plane_normal.dot(next);
        let current_inside =
            if keep_positive {
                current_dot >= 0.0
            } else {
                current_dot <= 0.0
            };
        let next_inside =
            if keep_positive {
                next_dot >= 0.0
            } else {
                next_dot <= 0.0
            };
        if current_inside {
            output.push(current);
        }
        if current_inside != next_inside
        {
            let denom =
                current_dot - next_dot;
            if denom.abs() > 1e-6 {
                let t =
                    current_dot / denom;
                let point = current
                    + (next - current)
                        * t;
                output.push(point);
            }
        }
    }
    output
}

fn polygon_to_triangles(
    polygon: &[Vec3],
) -> Vec<[Vec3; 3]> {
    let mut triangles = Vec::new();
    if polygon.len() < 3 {
        return triangles;
    }
    for i in 1..polygon.len() - 1 {
        triangles.push([
            polygon[0],
            polygon[i],
            polygon[i + 1],
        ]);
    }
    triangles
}

fn plane_intersection_polygon(
    vertices: &[Vec3; 8],
    normal: Vec3,
) -> Vec<Vec3> {
    let mut points = Vec::new();
    for &(a_idx, b_idx) in
        CUBE_EDGE_PAIRS
    {
        let a = vertices[a_idx];
        let b = vertices[b_idx];
        let da = normal.dot(a);
        let db = normal.dot(b);
        if (da >= 0.0 && db <= 0.0)
            || (da <= 0.0 && db >= 0.0)
        {
            let denom = da - db;
            if denom.abs() < 1e-6 {
                push_unique_point(
                    &mut points,
                    a,
                );
                push_unique_point(
                    &mut points,
                    b,
                );
                continue;
            }
            let t = da / denom;
            let point = a + (b - a) * t;
            push_unique_point(
                &mut points,
                point,
            );
        }
    }
    points
}

fn build_cap_triangles(
    polygon: &[Vec3],
    normal: Vec3,
    invert: bool,
) -> Vec<[Vec3; 3]> {
    let mut triangles = Vec::new();
    if polygon.len() < 3 {
        return triangles;
    }
    let sorted =
        sort_polygon(polygon, normal);
    if sorted.len() < 3 {
        return triangles;
    }
    let center =
        sorted.iter().copied().fold(
            Vec3::ZERO,
            |acc, p| acc + p,
        ) / (sorted.len() as f32);
    for i in 0..sorted.len() {
        let a = sorted[i];
        let b = sorted
            [(i + 1) % sorted.len()];
        if invert {
            triangles
                .push([center, b, a]);
        } else {
            triangles
                .push([center, a, b]);
        }
    }
    triangles
}

fn sort_polygon(
    points: &[Vec3],
    normal: Vec3,
) -> Vec<Vec3> {
    let axis = if normal
        .cross(Vec3::Y)
        .length_squared()
        > 1e-5
    {
        normal
            .cross(Vec3::Y)
            .normalize()
    } else {
        normal
            .cross(Vec3::X)
            .normalize()
    };
    let bitangent =
        normal.cross(axis).normalize();
    let mut sorted: Vec<_> =
        points.to_vec();
    sorted.sort_by(|a, b| {
        let a_angle = a
            .dot(bitangent)
            .atan2(a.dot(axis));
        let b_angle = b
            .dot(bitangent)
            .atan2(b.dot(axis));
        a_angle
            .partial_cmp(&b_angle)
            .unwrap_or(
            std::cmp::Ordering::Equal,
        )
    });
    sorted
}

fn ensure_cached_mesh(
    settings: &MeshCacheSettings,
    meshes: &mut Assets<Mesh>,
    triangles: &[[Vec3; 3]],
    key: &str,
    max_dim: f32,
    outward_hint: Vec3,
) -> (
    Handle<Mesh>,
    Vec<[f32; 3]>,
    Vec<[u32; 3]>,
) {
    if let Ok((mesh, data)) =
        load_cached_mesh(settings, key)
    {
        let handle = meshes.add(mesh);
        let positions =
            data.positions().clone();
        let indices = data
            .indices()
            .map(|idx| {
                u32_vec_to_triangles(
                    idx,
                )
            })
            .unwrap_or_default();
        return (
            handle, positions, indices,
        );
    }
    let (mesh, positions, indices, _) =
        build_mesh_data(triangles, max_dim, outward_hint);
    if let Err(err) =
        cache_mesh(settings, key, &mesh)
    {
        warn!(
            "Failed to cache cut mesh '{key}': {err}"
        );
    }
    let handle = meshes.add(mesh);
    (handle, positions, indices)
}

fn build_mesh_data(
    triangles: &[[Vec3; 3]],
    max_dim: f32,
    outward_hint: Vec3,
) -> (
    Mesh,
    Vec<[f32; 3]>,
    Vec<[u32; 3]>,
    Vec<u32>,
) {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut tri_indices = Vec::new();
    let mut flat_indices = Vec::new();
    let mut uvs = Vec::new();
    let mut next_index = 0u32;
    for tri in triangles {
        let mut tri_vertices = *tri;
        let center =
            (tri_vertices[0]
                + tri_vertices[1]
                + tri_vertices[2])
                / 3.0;
        let mut edge1 =
            tri_vertices[1]
                - tri_vertices[0];
        let mut edge2 =
            tri_vertices[2]
                - tri_vertices[0];
        let mut normal =
            edge1.cross(edge2);
        if normal.length_squared()
            < 1e-6
        {
            continue;
        }
        normal = normal.normalize();
        let dot_center = normal.dot(center);
        let mut reference = if center
            .length_squared()
            > 1e-6
            && dot_center.abs() > 1e-4
        {
            center
        } else {
            outward_hint
        };
        if reference.length_squared()
            < 1e-6
        {
            reference = Vec3::Y;
        }
        if normal.dot(reference)
            < 0.0
        {
            tri_vertices.swap(1, 2);
            edge1 =
                tri_vertices[1]
                    - tri_vertices[0];
            edge2 =
                tri_vertices[2]
                    - tri_vertices[0];
            normal =
                edge1.cross(edge2);
            if normal.length_squared()
                < 1e-6
            {
                continue;
            }
            normal = normal.normalize();
        }
        for vertex in tri_vertices.iter() {
            positions.push([
                vertex.x, vertex.y,
                vertex.z,
            ]);
            normals.push([
                normal.x, normal.y,
                normal.z,
            ]);
            uvs.push(project_uv(
                normal,
                *vertex,
                max_dim,
            ));
        }
        tri_indices.push([
            next_index,
            next_index + 1,
            next_index + 2,
        ]);
        flat_indices.push(next_index);
        flat_indices
            .push(next_index + 1);
        flat_indices
            .push(next_index + 2);
        next_index += 3;
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        positions.clone(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals,
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        uvs.clone(),
    );
    mesh.insert_indices(Indices::U32(
        flat_indices.clone(),
    ));
    (
        mesh,
        positions,
        tri_indices,
        flat_indices,
    )
}

fn u32_vec_to_triangles(
    indices: &[u32],
) -> Vec<[u32; 3]> {
    let mut triangles = Vec::new();
    let mut idx = 0;
    while idx + 2 < indices.len() {
        triangles.push([
            indices[idx],
            indices[idx + 1],
            indices[idx + 2],
        ]);
        idx += 3;
    }
    triangles
}

fn positions_to_points(
    positions: &[[f32; 3]],
) -> Vec<Vec3> {
    positions
        .iter()
        .map(|p| {
            Vec3::new(p[0], p[1], p[2])
        })
        .collect()
}

fn project_uv(
    normal: Vec3,
    vertex: Vec3,
    max_dim: f32,
) -> [f32; 2] {
    let abs = Vec3::new(
        normal.x.abs(),
        normal.y.abs(),
        normal.z.abs(),
    );
    let coords = if abs.x >= abs.y && abs.x >= abs.z {
        [vertex.z, vertex.y]
    } else if abs.y >= abs.z {
        [vertex.x, vertex.z]
    } else {
        [vertex.x, vertex.y]
    };
    let denom = (max_dim * 2.0).max(1e-6);
    [
        coords[0] / denom + 0.5,
        coords[1] / denom + 0.5,
    ]
}

fn spawn_half_entity(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    collider: Collider,
    cuttable: &CuttableShape,
    transform: &Transform,
) {
    let mut entity = commands.spawn((
        Name::new("cut_half"),
        SceneEntityTag,
        Mesh3d(mesh),
        MeshMaterial3d(
            cuttable.material.clone(),
        ),
        *transform,
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
    if let Some(physics) =
        &cuttable.physics
    {
        if physics.enabled {
            let rigid_body =
                resolve_rigid_body(
                    &physics.body_type,
                );
            entity.insert((
                rigid_body,
                collider,
                Restitution::coefficient(physics.restitution),
                Friction::coefficient(physics.friction),
                DespawnOutsideBounds,
            ));
            if matches!(
                rigid_body,
                RigidBody::Dynamic
            ) && physics.mass > 0.0
            {
                entity.insert(AdditionalMassProperties::Mass((physics.mass * 0.5).max(0.0001)));
            }
        }
    }
}

fn resolve_rigid_body(
    body_type: &str,
) -> RigidBody {
    match body_type.trim().to_ascii_lowercase().as_str() {
        "fixed" | "static" => RigidBody::Fixed,
        "kinematic_position" | "kinematic_position_based" => RigidBody::KinematicPositionBased,
        "kinematic_velocity" | "kinematic_velocity_based" => RigidBody::KinematicVelocityBased,
        _ => RigidBody::Dynamic,
    }
}

fn format_key(value: f32) -> String {
    let rounded = (value * 1000.0)
        .round()
        / 1000.0;
    format!("{rounded:.3}")
        .replace('.', "_")
}

fn push_unique_point(
    points: &mut Vec<Vec3>,
    point: Vec3,
) {
    if !points.iter().any(|existing| {
        (existing - point)
            .length_squared()
            < 1e-6
    }) {
        points.push(point);
    }
}
