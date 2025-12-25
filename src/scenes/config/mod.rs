#![allow(unused_imports)]

mod active;
mod actions;
mod bounds;
mod camera;
mod colors;
mod combo_entity;
mod entity;
mod combos;
mod input;
mod light;
mod overlay;
mod paths;
mod physics;
mod shapes;
mod stack;
mod sun;
mod skybox;
mod transforms;

pub use active::{ActiveScene, OVERLAY_ROOT, SCENE_ROOT};
pub use actions::{FovActionConfig, ShootActionConfig, SprintActionConfig, ZoomActionConfig};
pub use bounds::BoundingBoxConfig;
pub use camera::{CameraConfig, TransformConfig};
pub use colors::{
    default_circle_color_name, default_circle_rgb, default_circle_radius, default_color_name,
    default_color_rgb, parse_color,
};
pub use combo_entity::{AttachConfig, ComboPart, ComboPhysics, ComboStackConfig, ComboTemplate};
pub use entity::{
    EntityOverrides, EntityTemplate, LightComponent, LightOverridesConfig, PhysicsOverrides,
    ShapeConfig, ShapeKind, ShapeOverrides,
    TransformConfig as EntityTransformConfig, TransformOverrides,
};
pub use combos::PillarComboConfig;
pub use input::{
    ActionBindingConfig, CameraInputConfig, CameraRotationConfig, InputConfig, MovementConfig,
    OverlayInputConfig,
};
pub use light::{LightConfig, LightEntry, LightKind, LightOverrides};
pub use overlay::{
    ImageFit, ImageOverlay, OverlayAnchor, OverlayCommon, OverlayConfig, OverlayElement,
    OverlayOffset, TextOverlay,
};
pub use paths::{
    action_config_path, bounding_box_config_path, camera_config_path, circle_config_path,
    cube_config_path, input_config_path, light_config_path, overlay_config_path,
    pillar_combo_config_path, rectangle_config_path, skybox_config_path, sphere_config_path,
    sun_config_path, top_light_config_path,
};
pub use physics::PhysicsConfig;
pub use shapes::{CircleConfig, CubeConfig, RectangleConfig, RectangleOverrides, SphereConfig};
pub use stack::RectangleStackConfig;
pub use skybox::SkyboxConfig;
pub use sun::SunConfig;
pub use transforms::{CubeRotationConfig, DimensionsConfig, PositionConfig, SizeConfig, Vec3Config};
