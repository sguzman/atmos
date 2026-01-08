use std::path::{Path, PathBuf};
use std::fs;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext, RenderAssetUsages},
    log::{info, warn},
    prelude::{AssetServer, Assets, Handle, Mesh, Resource},
    render::render_resource::PrimitiveTopology,
};
use bevy_mesh::{Indices, MeshVertexAttribute, VertexAttributeValues};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::scenes::config::{EntityTemplate, ShapeConfig, ShapeKind, SCENE_FS_ROOT};
use crate::scenes::world::WorldConfig;

#[derive(Resource, Debug, Clone)]
pub struct MeshCacheSettings {
    pub allow_runtime: bool,
    pub cache_root: PathBuf,
    pub asset_prefix: String,
}

impl Default for MeshCacheSettings {
    fn default() -> Self {
        Self {
            allow_runtime: false,
            cache_root: PathBuf::from("assets/.cache/meshes"),
            asset_prefix: ".cache/meshes".to_string(),
        }
    }
}

impl MeshCacheSettings {
    pub fn new(allow_runtime: bool) -> Self {
        Self {
            allow_runtime,
            ..Default::default()
        }
    }

    pub fn asset_path_for_key(&self, key: &str) -> String {
        format!("{}/{key}.meshcache", self.asset_prefix)
    }

    pub fn fs_path_for_key(&self, key: &str) -> PathBuf {
        self.cache_root.join(format!("{key}.meshcache"))
    }
}

#[derive(Default)]
pub struct MeshCacheLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum MeshCacheError {
    #[error("Failed to read mesh cache: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to decode mesh cache: {0}")]
    Decode(#[from] bincode::Error),
    #[error("Unsupported mesh format: {0}")]
    Unsupported(String),
}

impl AssetLoader for MeshCacheLoader {
    type Asset = Mesh;
    type Settings = ();
    type Error = MeshCacheError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let data: CachedMeshData = bincode::deserialize(&bytes)?;
        data.to_mesh()
    }

    fn extensions(&self) -> &[&str] {
        &["meshcache"]
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedMeshData {
    topology: String,
    positions: Vec<[f32; 3]>,
    normals: Option<Vec<[f32; 3]>>,
    uvs: Option<Vec<[f32; 2]>>,
    indices: Option<Vec<u32>>,
}

impl CachedMeshData {
    fn from_mesh(mesh: &Mesh) -> Result<Self, MeshCacheError> {
        let positions = read_attribute_vec3(mesh, Mesh::ATTRIBUTE_POSITION)?;
        let normals = read_attribute_vec3(mesh, Mesh::ATTRIBUTE_NORMAL).ok();
        let uvs = read_attribute_vec2(mesh, Mesh::ATTRIBUTE_UV_0).ok();
        let indices = mesh.indices().map(indices_to_u32);
        Ok(Self {
            topology: topology_to_string(mesh.primitive_topology()),
            positions,
            normals,
            uvs,
            indices,
        })
    }

    fn to_mesh(self) -> Result<Mesh, MeshCacheError> {
        let topology = topology_from_string(&self.topology)?;
        let mut mesh = Mesh::new(topology, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        if let Some(normals) = self.normals {
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
        if let Some(uvs) = self.uvs {
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        }
        if let Some(indices) = self.indices {
            mesh.insert_indices(Indices::U32(indices));
        }
        Ok(mesh)
    }
}

pub fn load_or_generate_mesh_handle(
    settings: &MeshCacheSettings,
    shape: &ShapeConfig,
    meshes: &mut Assets<Mesh>,
    asset_server: &AssetServer,
) -> Handle<Mesh> {
    let key = mesh_key_for_shape(shape);
    let asset_path = settings.asset_path_for_key(&key);
    let handle = asset_server.load(asset_path);
    let fs_path = settings.fs_path_for_key(&key);
    if fs_path.exists() {
        return handle;
    }
    if !settings.allow_runtime {
        panic!(
            "Mesh cache missing for '{key}'. Run `bake` or enable --allow-runtime-mesh in dev."
        );
    }
    let mesh = build_mesh_from_shape(shape);
    if let Err(err) = save_mesh_cache(&fs_path, &mesh) {
        warn!("Failed to cache mesh '{key}': {err}");
    }
    meshes.add(mesh)
}

pub fn bake_all_meshes(settings: &MeshCacheSettings, shapes: &[ShapeConfig]) -> Result<(), MeshCacheError> {
    if !settings.cache_root.exists() {
        std::fs::create_dir_all(&settings.cache_root)?;
    }

    for shape in shapes {
        let key = mesh_key_for_shape(shape);
        let path = settings.fs_path_for_key(&key);
        if path.exists() {
            continue;
        }
        let mesh = build_mesh_from_shape(shape);
        save_mesh_cache(&path, &mesh)?;
        info!("Cached mesh {key} -> {}", path.display());
    }

    Ok(())
}

pub fn bake_meshes(scene: Option<&str>, settings: &MeshCacheSettings) -> Result<(), MeshCacheError> {
    let scenes = resolve_scenes(scene)?;
    let mut shapes = Vec::new();
    for scene_name in scenes {
        let entities_root = Path::new(SCENE_FS_ROOT)
            .join(&scene_name)
            .join("entities");
        if !entities_root.exists() {
            continue;
        }
        collect_shapes(&entities_root, &mut shapes)?;
        collect_world_shapes(&scene_name, &mut shapes)?;
    }
    bake_all_meshes(settings, &shapes)
}

fn save_mesh_cache(path: &Path, mesh: &Mesh) -> Result<(), MeshCacheError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let data = CachedMeshData::from_mesh(mesh)?;
    let bytes = bincode::serialize(&data)?;
    std::fs::write(path, bytes)?;
    Ok(())
}

fn collect_shapes(root: &Path, shapes: &mut Vec<ShapeConfig>) -> Result<(), MeshCacheError> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_shapes(&path, shapes)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }
        let contents = fs::read_to_string(&path)?;
        let template: EntityTemplate = toml::from_str(&contents).map_err(|err| {
            MeshCacheError::Unsupported(format!(
                "Failed to parse {}: {err}",
                path.display()
            ))
        })?;
        if let Some(shape) = template.shape {
            shapes.push(shape);
        }
    }
    Ok(())
}

fn collect_world_shapes(scene: &str, shapes: &mut Vec<ShapeConfig>) -> Result<(), MeshCacheError> {
    let world_path = Path::new(SCENE_FS_ROOT).join(scene).join("world.toml");
    if !world_path.exists() {
        return Ok(());
    }
    let contents = fs::read_to_string(&world_path)?;
    let world: WorldConfig = toml::from_str(&contents).map_err(|err| {
        MeshCacheError::Unsupported(format!(
            "Failed to parse {}: {err}",
            world_path.display()
        ))
    })?;
    if let Some(sun) = world.sun {
        shapes.push(ShapeConfig {
            kind: ShapeKind::Sphere,
            color: Some(sun.color),
            dimensions: None,
            radius: Some(sun.size),
        });
    }
    Ok(())
}

fn resolve_scenes(scene: Option<&str>) -> Result<Vec<String>, MeshCacheError> {
    if let Some(scene) = scene {
        return Ok(vec![scene.to_string()]);
    }
    let mut scenes = Vec::new();
    let root = Path::new(SCENE_FS_ROOT);
    if !root.exists() {
        return Ok(scenes);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                scenes.push(name.to_string());
            }
        }
    }
    Ok(scenes)
}

fn build_mesh_from_shape(shape: &ShapeConfig) -> Mesh {
    match shape.kind {
        ShapeKind::Box => {
            let dimensions = shape.dimensions.clone().unwrap_or_default();
            Mesh::from(bevy::math::primitives::Cuboid::new(
                dimensions.width,
                dimensions.height,
                dimensions.depth,
            ))
        }
        ShapeKind::Sphere => {
            let radius = shape.radius.unwrap_or(0.5);
            Mesh::from(bevy::math::primitives::Sphere::new(radius))
        }
        ShapeKind::Circle => {
            let radius = shape.radius.unwrap_or(4.0);
            Mesh::from(bevy::math::primitives::Circle::new(radius))
        }
    }
}

fn mesh_key_for_shape(shape: &ShapeConfig) -> String {
    match shape.kind {
        ShapeKind::Box => {
            let dimensions = shape.dimensions.clone().unwrap_or_default();
            format!(
                "box_w{}_h{}_d{}",
                format_key(dimensions.width),
                format_key(dimensions.height),
                format_key(dimensions.depth)
            )
        }
        ShapeKind::Sphere => {
            let radius = shape.radius.unwrap_or(0.5);
            format!("sphere_r{}", format_key(radius))
        }
        ShapeKind::Circle => {
            let radius = shape.radius.unwrap_or(4.0);
            format!("circle_r{}", format_key(radius))
        }
    }
}

fn format_key(value: f32) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    format!("{rounded:.3}").replace('.', "_")
}

fn read_attribute_vec3(
    mesh: &Mesh,
    attribute: MeshVertexAttribute,
) -> Result<Vec<[f32; 3]>, MeshCacheError> {
    match mesh.attribute(attribute) {
        Some(VertexAttributeValues::Float32x3(values)) => Ok(values.clone()),
        Some(other) => Err(MeshCacheError::Unsupported(format!(
            "Expected Float32x3, got {other:?}"
        ))),
        None => Err(MeshCacheError::Unsupported("Missing attribute".to_string())),
    }
}

fn read_attribute_vec2(
    mesh: &Mesh,
    attribute: MeshVertexAttribute,
) -> Result<Vec<[f32; 2]>, MeshCacheError> {
    match mesh.attribute(attribute) {
        Some(VertexAttributeValues::Float32x2(values)) => Ok(values.clone()),
        Some(other) => Err(MeshCacheError::Unsupported(format!(
            "Expected Float32x2, got {other:?}"
        ))),
        None => Err(MeshCacheError::Unsupported("Missing attribute".to_string())),
    }
}

fn indices_to_u32(indices: &Indices) -> Vec<u32> {
    match indices {
        Indices::U16(values) => values.iter().map(|v| *v as u32).collect(),
        Indices::U32(values) => values.clone(),
    }
}

fn topology_to_string(topology: PrimitiveTopology) -> String {
    match topology {
        PrimitiveTopology::PointList => "point_list",
        PrimitiveTopology::LineList => "line_list",
        PrimitiveTopology::LineStrip => "line_strip",
        PrimitiveTopology::TriangleList => "triangle_list",
        PrimitiveTopology::TriangleStrip => "triangle_strip",
    }
    .to_string()
}

fn topology_from_string(value: &str) -> Result<PrimitiveTopology, MeshCacheError> {
    match value {
        "point_list" => Ok(PrimitiveTopology::PointList),
        "line_list" => Ok(PrimitiveTopology::LineList),
        "line_strip" => Ok(PrimitiveTopology::LineStrip),
        "triangle_list" => Ok(PrimitiveTopology::TriangleList),
        "triangle_strip" => Ok(PrimitiveTopology::TriangleStrip),
        other => Err(MeshCacheError::Unsupported(format!(
            "Unknown topology '{other}'"
        ))),
    }
}
