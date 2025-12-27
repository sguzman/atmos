use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MaterialConfig {
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub base_color: Option<String>,
    #[serde(default)]
    pub base_color_texture: Option<String>,
    #[serde(default)]
    pub opacity: Option<f32>,
    #[serde(default)]
    pub metallic: Option<f32>,
    #[serde(default)]
    pub roughness: Option<f32>,
    #[serde(default)]
    pub reflectance: Option<f32>,
    #[serde(default)]
    pub specular_tint: Option<String>,
    #[serde(default)]
    pub emissive_color: Option<String>,
    #[serde(default)]
    pub emissive_intensity: Option<f32>,
    #[serde(default)]
    pub emissive_texture: Option<String>,
    #[serde(default)]
    pub normal_map: Option<String>,
    #[serde(default)]
    pub flip_normal_map_y: Option<bool>,
    #[serde(default)]
    pub metallic_roughness_texture: Option<String>,
    #[serde(default)]
    pub occlusion_texture: Option<String>,
    #[serde(default)]
    pub alpha_mode: Option<String>,
    #[serde(default)]
    pub alpha_cutoff: Option<f32>,
    #[serde(default)]
    pub unlit: Option<bool>,
    #[serde(default)]
    pub double_sided: Option<bool>,
    #[serde(default)]
    pub clearcoat: Option<f32>,
    #[serde(default)]
    pub clearcoat_roughness: Option<f32>,
    #[serde(default)]
    pub ior: Option<f32>,
    #[serde(default)]
    pub specular_transmission: Option<f32>,
    #[serde(default)]
    pub diffuse_transmission: Option<f32>,
    #[serde(default)]
    pub thickness: Option<f32>,
    #[serde(default)]
    pub attenuation_color: Option<String>,
    #[serde(default)]
    pub attenuation_distance: Option<f32>,
}
