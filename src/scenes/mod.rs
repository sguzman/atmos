#![allow(clippy::collapsible_if)]
#![allow(clippy::default_constructed_unit_structs)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::manual_flatten)]
#![allow(clippy::manual_ignore_case_cmp)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unwrap_or_default)]

mod bounds;
mod config;
mod entities;
mod input;
mod loaders;
mod menu;
mod mesh_cache;
mod spawn;
mod state;
mod toml_asset;
mod world;

pub use loaders::TomlCache;
pub use menu::MenuPlugin;
pub use mesh_cache::{
    MeshCacheLoader, MeshCacheSettings, bake_meshes, load_or_generate_mesh_handle,
};
pub use spawn::ScenePlugin;
pub use state::AppState;
pub use toml_asset::{TomlAsset, TomlAssetLoader};
pub use world::WorldConfig;
