// src/canvas/canvas_rotation.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let param = m.get(1).map(|s| s.as_str()).unwrap_or("");

    if param == "E" {
        // RO, E -> RotationEnd
        canvas_lib::SpcCommand::RotationEnd
    } else {
        // RO, @R1, 45 -> RotationStart(ID, angle)
        let id = param.to_string();
        let angle = m.get(2)
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0);
        
        canvas_lib::SpcCommand::RotationStart(id, angle)
    }
}
