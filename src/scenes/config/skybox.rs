use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SkyboxConfig {
    #[serde(default = "default_sky_color")]
    pub color: String,
}

impl Default for SkyboxConfig {
    fn default() -> Self {
        Self {
            color: default_sky_color(),
        }
    }
}

fn default_sky_color() -> String {
    "lightblue".to_string()
}
