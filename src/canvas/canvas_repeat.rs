// src/canvas/canvas_place.rs

use crate::canvas::canvas_lib::{SpcCommand, SpcRepeat};

pub fn calc(row: &[String]) -> SpcCommand {
    let group_id = row.get(1).cloned().unwrap_or_default();
    
    let count_x = row.get(2)
        .and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let count_y = row.get(3)
        .and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let base_x = row.get(4)
        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
    let base_y = row.get(5)
        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
    let step_x = row.get(6)
        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
    let step_y = row.get(7)
        .and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);

    SpcCommand::Repeat(SpcRepeat {
        group_id,
        count_x,
        count_y,
        base_x,
        base_y,
        step_x,
        step_y,
    })
}
