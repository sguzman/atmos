mod actions;
mod active;
mod bounds;
mod camera;
mod colors;
mod combo_entity;
mod dialogue;
mod entity;
mod input;
mod light;
mod material;
mod overlay;
mod paths;
mod physics;
mod render;
mod skybox;
mod sun;
mod transforms;

pub use actions::{
    ActionConfig, ActionTriggerConfig,
    ActionsConfig, CutActionConfig,
    CutActivationMode,
    GrabActionConfig,
    GrenadeActionConfig,
    JumpActionConfig,
    NoclipActionConfig,
    PauseActionConfig,
    ShootActionConfig,
    SprintActionConfig, TriggerMode,
    VolumeShapeKind, VolumeTriggerMode,
    ZoomActionConfig,
};
pub use active::{
    ActiveScene, DIALOGUE_ROOT,
    OVERLAY_ROOT, SCENE_FS_ROOT,
    SCENE_ROOT,
};
pub use bounds::BoundingBoxConfig;
pub use camera::CameraConfig;
pub use colors::{
    default_circle_color_name,
    default_circle_rgb,
    default_color_name,
    default_color_rgb, parse_color,
};
pub use combo_entity::{
    ComboPart, ComboPhysics,
    ComboStackConfig, ComboTemplate,
};
pub use dialogue::{
    DialogueConfig, DialogueNode,
    DialogueOption,
};
pub use entity::{
    EntityOverrides, EntityTemplate,
    LightComponent,
    LightOverridesConfig,
    PhysicsOverrides, ShapeConfig,
    ShapeKind, ShapeOverrides,
    TransformConfig as EntityTransformConfig,
    TransformOverrides,
};
pub use input::{
    CameraRotationConfig, InputConfig,
    MovementConfig, OverlayInputConfig,
};
pub use light::{
    LightEntry, LightKind,
};
pub use material::MaterialConfig;
pub use overlay::{
    OverlayAnchor, OverlayConfig,
    OverlayElement, TextOverlay,
};
pub use paths::{
    action_config_path,
    actions_config_path,
    dialogue_config_path,
    input_config_path,
    overlay_config_path,
};
pub use physics::{
    PhysicsConfig, WorldPhysicsConfig,
};
pub use render::{
    BloomConfig, DlssConfig, FogConfig,
    FogFalloffConfig, RayTracingConfig,
    RenderConfig,
};
pub use skybox::SkyboxConfig;
pub use sun::SunConfig;
pub use transforms::Vec3Config;
