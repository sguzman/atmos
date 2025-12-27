mod active;
mod actions;
mod bounds;
mod camera;
mod colors;
mod combo_entity;
mod entity;
mod input;
mod light;
mod material;
mod overlay;
mod paths;
mod physics;
mod render;
mod sun;
mod skybox;
mod transforms;

pub use active::{ActiveScene, OVERLAY_ROOT, SCENE_ROOT};
pub use actions::{ShootActionConfig, SprintActionConfig, ZoomActionConfig};
pub use bounds::BoundingBoxConfig;
pub use camera::CameraConfig;
pub use colors::{
    default_circle_color_name, default_circle_rgb, default_color_name, default_color_rgb,
    parse_color,
};
pub use combo_entity::{ComboPart, ComboPhysics, ComboStackConfig, ComboTemplate};
pub use entity::{
    EntityOverrides, EntityTemplate, LightComponent, LightOverridesConfig, PhysicsOverrides,
    ShapeConfig, ShapeKind, ShapeOverrides,
    TransformConfig as EntityTransformConfig, TransformOverrides,
};
pub use input::{CameraRotationConfig, InputConfig, MovementConfig, OverlayInputConfig};
pub use light::{LightEntry, LightKind};
pub use material::MaterialConfig;
pub use overlay::{
    OverlayAnchor, OverlayConfig, OverlayElement, TextOverlay,
};
pub use render::{BloomConfig, FogConfig, FogFalloffConfig, RenderConfig};
pub use paths::{
    action_config_path, input_config_path, overlay_config_path,
};
pub use physics::PhysicsConfig;
pub use skybox::SkyboxConfig;
pub use sun::SunConfig;
pub use transforms::Vec3Config;
