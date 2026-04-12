// src/canvas/canvas_group.rs

use crate::canvas::canvas_lib;

pub fn calc(m: &[String]) -> canvas_lib::SpcCommand {
    let param = m.get(1).map(|s| s.as_str()).unwrap_or("");

    if param == "E" {
        // GR, E -> GroupEnd
        canvas_lib::SpcCommand::GroupEnd
    } else {
        // GR, @G1 -> GroupStart(ID)
        let id = param.to_string();
        canvas_lib::SpcCommand::GroupStart(id)
    }
}
