#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct OverlayConfig {
    #[serde(default)]
    pub elements: Vec<OverlayElement>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OverlayElement {
    Text(TextOverlay),
    Image(ImageOverlay),
}

impl Default for OverlayElement {
    fn default() -> Self {
        OverlayElement::Text(TextOverlay::default())
    }
}

#[derive(Debug, Deserialize)]
pub struct TextOverlay {
    #[serde(flatten)]
    pub common: OverlayCommon,
    pub content: String,
    #[serde(default = "default_text_color")]
    pub color: String,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default)]
    pub font_weight: Option<String>,
}

impl Default for TextOverlay {
    fn default() -> Self {
        Self {
            common: OverlayCommon::default(),
            content: String::new(),
            color: default_text_color(),
            font_size: default_font_size(),
            font_family: default_font_family(),
            font_weight: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageOverlay {
    #[serde(flatten)]
    pub common: OverlayCommon,
    pub source: String,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default)]
    pub height: Option<f32>,
    #[serde(default)]
    pub fit: Option<ImageFit>,
}

impl Default for ImageOverlay {
    fn default() -> Self {
        Self {
            common: OverlayCommon::default(),
            source: String::new(),
            width: None,
            height: None,
            fit: None,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct OverlayCommon {
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default)]
    pub anchor: OverlayAnchor,
    #[serde(default)]
    pub offset: OverlayOffset,
    #[serde(default)]
    pub rotation_deg: f32,
    #[serde(default = "default_scale")]
    pub scale: f32,
}

#[derive(Debug, Deserialize)]
pub struct OverlayOffset {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl Default for OverlayOffset {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

impl Default for OverlayAnchor {
    fn default() -> Self {
        OverlayAnchor::TopLeft
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageFit {
    Contain,
    Cover,
    Fill,
    FitWidth,
    FitHeight,
}

fn default_visible() -> bool {
    true
}

fn default_opacity() -> f32 {
    1.0
}

fn default_scale() -> f32 {
    1.0
}

fn default_text_color() -> String {
    "white".to_string()
}

fn default_font_size() -> f32 {
    16.0
}

fn default_font_family() -> String {
    "sans-serif".to_string()
}
