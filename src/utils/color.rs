// src/utils/color.rs

use eframe::egui::Color32;

pub fn get_standard_color(name: &str) -> (u8, u8, u8, u8) {
    match name {
        "red"   => (255, 0, 0, 255),
        "green" => (0, 255, 0, 255),
        "blue"  => (0, 0, 255, 255),
        "white" => (255, 255, 255, 255),
        _       => (0, 0, 0, 255), // 見つからなければ黒
    }
}

pub fn get_color(name: &str) -> Color32 {
    match name {
        "black"        => Color32::BLACK,
        "dark_gray"    => Color32::DARK_GRAY,
        "gray"         => Color32::GRAY,
        "light_gray"   => Color32::LIGHT_GRAY,
        "white"        => Color32::WHITE,
        "brown"        => Color32::BROWN,
        "dark_red"     => Color32::DARK_RED,
        "red"          => Color32::RED,
        "light_red"    => Color32::LIGHT_RED,
        "cyan"         => Color32::CYAN,
        "magenta"      => Color32::MAGENTA,
        "yellow"       => Color32::YELLOW,
        "orange"       => Color32::ORANGE,
        "light_yellow" => Color32::LIGHT_YELLOW,
        "khaki"        => Color32::KHAKI,
        "dark_green"   => Color32::DARK_GREEN,
        "green"        => Color32::GREEN,
        "light_green"  => Color32::LIGHT_GREEN,
        "dark_blue"    => Color32::DARK_BLUE,
        "blue"         => Color32::BLUE,
        "light_blue"   => Color32::LIGHT_BLUE,
        "purple"       => Color32::PURPLE,
        "gold"         => Color32::GOLD,
        "transparent"  => Color32::TRANSPARENT,
        "debug_color"  => Color32::DEBUG_COLOR,
        _              => Color32::DEBUG_COLOR,
    }
}
