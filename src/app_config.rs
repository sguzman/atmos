use std::{fs, path::Path};

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    time::TimeUpdateStrategy,
    window::{PresentMode, WindowPlugin, WindowResolution},
};
use serde::Deserialize;

const CONFIG_PATH: &str = "assets/config.toml";

#[derive(Resource, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub mode: AppMode,
    pub fps_limit: Option<f64>,
    pub log_level: Option<String>,
    pub window: WindowConfig,
    pub msaa_samples: Option<u32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mode: AppMode::Dev,
            fps_limit: Some(60.0),
            log_level: Some("debug".to_string()),
            window: WindowConfig::default(),
            msaa_samples: Some(4),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppMode {
    Dev,
    Prod,
}

impl Default for AppMode {
    fn default() -> Self {
        AppMode::Dev
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentModeConfig,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Atmos".to_string(),
            width: 1280,
            height: 720,
            present_mode: PresentModeConfig::AutoVsync,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentModeConfig {
    AutoVsync,
    AutoNoVsync,
    Fifo,
    Mailbox,
    Immediate,
}

impl Default for PresentModeConfig {
    fn default() -> Self {
        PresentModeConfig::AutoVsync
    }
}

impl PresentModeConfig {
    fn to_bevy(&self) -> PresentMode {
        match self {
            PresentModeConfig::AutoVsync => PresentMode::AutoVsync,
            PresentModeConfig::AutoNoVsync => PresentMode::AutoNoVsync,
            PresentModeConfig::Fifo => PresentMode::Fifo,
            PresentModeConfig::Mailbox => PresentMode::Mailbox,
            PresentModeConfig::Immediate => PresentMode::Immediate,
        }
    }
}

pub fn load_app_config() -> AppConfig {
    let default_config = AppConfig::default();
    if !Path::new(CONFIG_PATH).exists() {
        warn!(
            "Config file '{}' not found; using defaults.",
            CONFIG_PATH
        );
        return default_config;
    }

    let contents = match fs::read_to_string(CONFIG_PATH) {
        Ok(text) => text,
        Err(err) => {
            warn!("Failed to read {}: {err}; using defaults.", CONFIG_PATH);
            return default_config;
        }
    };

    match toml::from_str::<AppConfig>(&contents) {
        Ok(cfg) => cfg,
        Err(err) => {
            warn!("Failed to parse {}: {err}; using defaults.", CONFIG_PATH);
            default_config
        }
    }
}

impl AppConfig {
    pub fn to_log_plugin(&self) -> LogPlugin {
        let mut log = LogPlugin::default();
        if let Some(level) = self.log_level() {
            log.level = level;
        }
        log
    }

    pub fn to_window_plugin(&self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: self.window.title.clone(),
                resolution: WindowResolution::new(self.window.width, self.window.height),
                present_mode: self.window.present_mode.to_bevy(),
                ..default()
            }),
            ..default()
        }
    }

    pub fn time_update_strategy(&self) -> Option<TimeUpdateStrategy> {
        self.fps_limit
            .and_then(|fps| if fps > 0.0 { Some(fps) } else { None })
            .map(|fps| TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(1.0 / fps)))
    }

    pub fn msaa_component(&self) -> Option<Msaa> {
        self.msaa_samples
            .and_then(|samples| match samples {
                1 => Some(Msaa::Off),
                2 => Some(Msaa::Sample2),
                4 => Some(Msaa::Sample4),
                8 => Some(Msaa::Sample8),
                _ => None,
            })
    }

    fn log_level(&self) -> Option<Level> {
        self.log_level.as_ref().and_then(|value| match value.to_ascii_lowercase().as_str() {
            "error" => Some(Level::ERROR),
            "warn" | "warning" => Some(Level::WARN),
            "info" => Some(Level::INFO),
            "debug" => Some(Level::DEBUG),
            "trace" => Some(Level::TRACE),
            _ => None,
        })
    }
}
