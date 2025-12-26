use bevy::log::warn;

pub fn default_color_name() -> String {
    "red".to_string()
}

pub fn default_color_rgb() -> [u8; 3] {
    parse_color(&default_color_name()).unwrap_or([255, 0, 0])
}

pub fn parse_color(color_name: &str) -> Option<[u8; 3]> {
    match csscolorparser::parse(color_name) {
        Ok(parsed) => {
            let [r, g, b, _a] = parsed.to_rgba8();
            Some([r, g, b])
        }
        Err(err) => {
            warn!("Failed to parse color '{color_name}': {err}");
            None
        }
    }
}

pub fn default_circle_color_name() -> String {
    "white".to_string()
}

pub fn default_circle_rgb() -> [u8; 3] {
    parse_color(&default_circle_color_name()).unwrap_or([255, 255, 255])
}
