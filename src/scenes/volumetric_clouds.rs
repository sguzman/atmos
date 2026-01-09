use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel};
use bevy::render::renderer::RenderContext;
use bevy::render::RenderApp;

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

#[derive(Clone, Debug, Hash, PartialEq, Eq, RenderLabel)]
pub struct CloudsRenderNodeLabel;

pub struct VolumetricCloudsPlugin;

impl Plugin for VolumetricCloudsPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(CloudsRenderNodeLabel, CloudsRenderNode::default());
        render_graph.add_node_edge(CloudsRenderNodeLabel, bevy::render::graph::CameraDriverLabel);
    }
}

#[derive(Default)]
struct CloudsRenderNode;

impl Node for CloudsRenderNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        _world: &World,
    ) -> Result<(), NodeRunError> {
        Ok(())
    }
}
