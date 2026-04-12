// src/canvas/canvas_place.rs

use crate::canvas::canvas_lib::{SpcCommand, SpcPlace};

pub fn calc(row: &[String]) -> SpcCommand {
    let group_id = row.get(1).cloned().unwrap_or_default();
    
    // 各数値をパース。存在しない場合はデフォルト値を設定
    let x = row.get(2)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    let y = row.get(3)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);

    SpcCommand::Place(SpcPlace {
        group_id,
        x,
        y,
    })
}
