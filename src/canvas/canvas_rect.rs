// src/canvas/canvas_rect.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let rect = canvas_lib::SpcRect {
        x: m.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        y: m.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        w: m.get(3).and_then(|s| s.parse().ok()).unwrap_or(10.0),
        h: m.get(4).and_then(|s| s.parse().ok()).unwrap_or(10.0),
        color: m.get(5).cloned(),
        width: m.get(6).and_then(|s| s.parse().ok()),
        fill_color: m.get(7).cloned(),
    };

    canvas_lib::SpcCommand::Rect(rect)
}
