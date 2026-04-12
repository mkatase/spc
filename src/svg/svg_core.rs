// src/svg/svg_core.rs

use crate::CommandMap;
use crate::error::AppError;
use crate::utils::{common, file};
use crate::canvas::canvas_lib::SpcCommand;
use crate::canvas::canvas_lib::CanvasState;
use crate::canvas::canvas_lib::DrawContext;
use crate::canvas::canvas_exec::SpcExecutor;
use crate::svg::svg_lib::SvgBackend;

pub fn main(name: String) -> Result<(), AppError> {
    println!("--- Svg main ---");

    let v: Vec<Vec<String>> = file::read_file(name)?;
    let m: CommandMap = common::make_map(v.clone());
    //let s = common::simplify_config(&m);
    let mut state = CanvasState::default();
    state.overwrite_from_map(&m);

    let mut ctx = DrawContext::default();

    let mut backend = SvgBackend::new(state.width, state.height);
    
    for row in v {
        if let Some(cmd) = SpcCommand::parse(&row) {
            backend.execute(cmd, &mut state, &mut ctx);
        }
    }

    let output = backend.finalize(&state.bg_color);
    let output_name = format!("{}.svg", state.file_name);

    std::fs::write(&output_name, output)?;

    println!("Svg saved as: {}", output_name);

    Ok(())
}
