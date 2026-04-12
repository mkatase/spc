// src/canvas/canvas_line.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let line = canvas_lib::SpcLine {
        p1: [
            m.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            m.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        ],
        p2: [
            m.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            m.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        ],
        color: m.get(5).cloned(), 
        width: m.get(6).and_then(|s| s.parse().ok()),
    };

    canvas_lib::SpcCommand::Line(line)
}
