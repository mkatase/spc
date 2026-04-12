// src/image/image_core.rs

use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use crate::CommandMap;
use crate::error::AppError;
use crate::utils::{common, file};
use crate::canvas::canvas_color;
use crate::canvas::canvas_lib::SpcCommand;
use crate::canvas::canvas_lib::CanvasState;
use crate::canvas::canvas_lib::DrawContext;
use crate::canvas::canvas_exec::SpcExecutor;

pub fn main(name: String) -> Result<(), AppError> {
    println!("--- Image main ---");

    let v: Vec<Vec<String>> = file::read_file(name)?;
    let m: CommandMap = common::make_map(v.clone());
    let mut state = CanvasState::default();
    state.overwrite_from_map(&m);

    let mut ctx = DrawContext::default();
    let mut img = RgbaImage::new(state.width as u32, state.height as u32);

    let bg_color = canvas_color::name_to_rgba(&state.bg_color); // または好みの色
    draw_filled_rect_mut(
        &mut img, 
        Rect::at(0, 0).of_size(state.width as u32, state.height as u32), 
        Rgba(bg_color)
    );

    for row in v {
        if let Some(cmd) = SpcCommand::parse(&row) {
            img.execute(cmd, &mut state, &mut ctx);
        }
    }

    let output_name = format!("{}.{}", state.file_name, state.format); //
    img.save(&output_name).expect("Failed to save image");
    
    println!("Image saved as: {}", output_name);

    Ok(())
}
