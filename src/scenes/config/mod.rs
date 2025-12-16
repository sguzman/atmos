#![allow(unused_imports)]

mod active;
mod camera;
mod colors;
mod combos;
mod input;
mod light;
mod overlay;
mod paths;
mod physics;
mod shapes;
mod sun;
mod transforms;

pub use active::{ActiveScene, OVERLAY_ROOT, SCENE_ROOT};
pub use camera::{CameraConfig, TransformConfig};
pub use colors::{
    default_circle_color_name, default_circle_rgb, default_circle_radius, default_color_name,
    default_color_rgb, parse_color,
};
pub use combos::PillarComboConfig;
pub use input::{CameraInputConfig, CameraRotationConfig, InputConfig, MovementConfig};
pub use input::OverlayInputConfig;
pub use light::{LightConfig, LightEntry, LightKind, LightOverrides};
pub use overlay::{
    ImageFit, ImageOverlay, OverlayAnchor, OverlayCommon, OverlayConfig, OverlayElement,
    OverlayOffset, TextOverlay,
};
pub use paths::{
    camera_config_path, circle_config_path, cube_config_path, input_config_path,
    light_config_path, overlay_config_path, pillar_combo_config_path, rectangle_config_path,
    sun_config_path, top_light_config_path,
};
pub use physics::PhysicsConfig;
pub use shapes::{CircleConfig, CubeConfig, RectangleConfig, RectangleOverrides};
pub use sun::SunConfig;
pub use transforms::{CubeRotationConfig, DimensionsConfig, PositionConfig, SizeConfig, Vec3Config};
