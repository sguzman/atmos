use bevy::prelude::*;

use crate::scenes::config::{RenderConfig, VolumetricCloudsConfig};

#[derive(Resource, Debug, Clone)]
#[allow(dead_code)]
pub struct SceneCloudsConfig {
    pub config: VolumetricCloudsConfig,
}

pub fn apply_clouds_settings(render: Option<&RenderConfig>, commands: &mut Commands) {
    let clouds = render.and_then(|render| render.clouds.as_ref());
    match clouds {
        Some(config) if config.enabled => {
            commands.insert_resource(SceneCloudsConfig {
                config: config.clone(),
            });
        }
        _ => {
            commands.remove_resource::<SceneCloudsConfig>();
        }
    }
}
