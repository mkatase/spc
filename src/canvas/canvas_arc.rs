// src/canvas/canvas_arc.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let arc = canvas_lib::SpcArc {
        center: [
            m.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            m.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        ],
        radius:
            m.get(3).and_then(|s| s.parse().ok()).unwrap_or(10.0),
        start_deg:
            m.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        end_deg:
            m.get(5).and_then(|s| s.parse().ok()).unwrap_or(360.0),
        color: m.get(6).cloned(),
        width: m.get(7).and_then(|s| s.parse().ok()),
        fill_color: m.get(8).cloned(),
    };

    canvas_lib::SpcCommand::Arc(arc)
}
