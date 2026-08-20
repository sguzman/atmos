mod light;
mod material;
mod merge;
mod shape;
mod template;

pub use template::spawn_entity_from_template;

pub(super) use light::spawn_light_component;
pub(super) use merge::{
    apply_transform_additive, apply_translation, merge_light, merge_physics, merge_shape,
};
pub(super) use shape::spawn_shape_instance;
