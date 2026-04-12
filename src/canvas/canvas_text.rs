// src/canvas/canvas_text.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let text = canvas_lib::SpcText {
        pos: [
            m.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            m.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        ],
        text:
            m.get(3).cloned().unwrap_or_default(),
        size:
            m.get(4).and_then(|s| s.parse::<f32>().ok()),
        color: m.get(5).cloned(),
        angle: 0.0,
    };

    canvas_lib::SpcCommand::Text(text)
}
