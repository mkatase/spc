// src/canvas/canvas_circle.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let circle = canvas_lib::SpcCircle {
        center: [
            m.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            m.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        ],
        radius:
            m.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        color: m.get(4).cloned(),
        width: m.get(5).and_then(|s| s.parse().ok()),
        fill_color: m.get(6).cloned(),
    };

    canvas_lib::SpcCommand::Circle(circle)
}
