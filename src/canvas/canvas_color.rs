// src/canvas/canvas_color.rs

use eframe::egui::Color32;

use crate::error::AppError;
use crate::canvas::canvas_lib::CanvasState;

pub fn to_svg_str(rgba: [u8; 4]) -> String {
    if rgba[3] == 0 {
        "none".to_string()
    } else {
        format!("rgb({}, {}, {})", rgba[0], rgba[1], rgba[2])
    }
}

pub fn resolve_color(name: &str, state: &CanvasState) -> [u8; 4] {
    let clean_name = match validate_color_name(name) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{}", e);
            return [0, 200, 0, 128]; // debug color
        }
    };

    if let Some(rgba) = state.custom_colors.get(&clean_name) {
        return *rgba;
    }

    name_to_rgba(&clean_name) 
}

pub fn validate_color_name(name: &str) -> Result<String, AppError> {
    if name.contains(' ') {
        return Err(
            AppError::InvalidFormat(format!("Invalid color name: '{}' (spaces are not allowed)", name)));
    }

    let lower = name.to_lowercase();
    if name != lower {
        eprintln!("Warning: Color name '{}' contained uppercase letters. Converted to '{}'.", name, lower);
    }

    Ok(lower)
}

pub fn hex_to_rgba(hex_str: &str) -> [u8; 4] {
    let s = hex_str.trim_start_matches('#');
    
    let val = u32::from_str_radix(s, 16).unwrap_or(0);

    if s.len() == 6 {
        let r = ((val >> 16) & 0xff) as u8;
        let g = ((val >> 8) & 0xff) as u8;
        let b = (val & 0xff) as u8;
        [r, g, b, 255]
    } else {
        let r = ((val >> 24) & 0xff) as u8;
        let g = ((val >> 16) & 0xff) as u8;
        let b = ((val >> 8) & 0xff) as u8;
        let a = (val & 0xff) as u8;
        [r, g, b, a]
    }
}

pub fn get_standard_color(name: &str) -> (u8, u8, u8, u8) {
    match name {
        "red"   => (255, 0, 0, 255),
        "green" => (0, 255, 0, 255),
        "blue"  => (0, 0, 255, 255),
        "white" => (255, 255, 255, 255),
        _       => (0, 0, 0, 255),
    }
}

pub fn get_color(name: &str) -> Color32 {
    let [r, g, b, a] = name_to_rgba(name);
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

pub fn name_to_rgba(name: &str) -> [u8; 4] {
    let clean_name = validate_color_name(name).unwrap_or_else(|_| "debug_color".to_string());

    match clean_name.as_str() {
        "black"        => [0, 0, 0, 255],
        "dark_gray"    => [169, 169, 169, 255],      // Web is 169, Color32 is 96
        "gray"         => [160, 160, 160, 255],
        "light_gray"   => [220, 220, 220, 255],
        "white"        => [255, 255, 255, 255],
        "brown"        => [165, 42, 42, 255],
        "dark_red"     => [139, 0, 0, 255],
        "red"          => [255, 0, 0, 255],
        "light_red"    => [255, 128, 128, 255],
        "cyan"         => [0, 255, 255, 255],
        "magenta"      => [255, 0, 255, 255], 
        "yellow"       => [255, 255, 0, 255],
        "orange"       => [255, 165, 0, 255],
        "light_yellow" => [255, 255, 224, 255],
        "khaki"        => [240, 230, 140, 255],
        "dark_green"   => [0, 100, 0, 255],
        "green"        => [0, 255, 0, 255],
        "light_green"  => [144, 238, 144, 255],
        "dark_blue"    => [0, 0, 139, 255],
        "blue"         => [0, 0, 255, 255],
        "light_blue"   => [173, 216, 230, 255],
        "purple"       => [128, 0, 128, 255],
        "gold"         => [255, 215, 0, 255],
        "ivory"        => [255, 255, 240, 255],  // addtional color
        "azure"        => [240, 255, 255, 255],
        "lavender"     => [230, 230, 250, 255],
        "navy"         => [0, 0, 128, 255],
        "sky_blue"     => [135, 206, 235, 255],
        "teal"         => [0, 128, 128, 255],
        "wheat"        => [245, 222, 179, 255],
        "maroon"       => [128, 0, 0, 255],
        "salmon"       => [250, 128, 114, 255],
        "coral"        => [255, 127, 80, 255],
        "crimson"      => [220, 20, 60, 255],
        "deep_pink"    => [255, 20, 147, 255],
        "hot_pink"     => [255, 105, 180, 255],
        "violet"       => [238, 130, 238, 255],
        "transparent"  => [0, 0, 0, 0], 
        "debug_color"  => [0, 200, 0, 128],
        _ if name.starts_with('#') => {
            hex_to_rgba(name)
        }
        _              => [0, 200, 0, 128],

    }
}
