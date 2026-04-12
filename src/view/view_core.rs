// src/view/view_core.rs

use eframe::egui;
use std::sync::Arc;

use crate::CommandMap;
use crate::utils::file;
use crate::utils::common;
use crate::view::view_lib::SpcApp;
use crate::canvas::canvas_lib::{CanvasState, DrawContext};
use crate::canvas::canvas_exec::SpcExecutor;
use crate::canvas::canvas_lib::SpcCommand;

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "noto_sans".to_owned(),
        Arc::new(egui::FontData::from_static(crate::canvas::canvas_lib::FONT_DATA)),
    );

    fonts.families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "noto_sans".to_owned());

    ctx.set_fonts(fonts);
}

pub fn draw(app: &mut SpcApp, ui: &mut egui::Ui) {

    let mut state = CanvasState::default();
    let mut ctx = DrawContext::default();

    state.overwrite_from_map(&app.map);

    let rect = ui.max_rect();
    let mut painter = ui.painter_at(rect);

    for row in &app.rows {
        if let Some(cmd) = SpcCommand::parse(row) {
            //println!("{:?}", row);
            painter.execute(cmd, &mut state, &mut ctx);
        }
    }
}

pub fn main(mode: &str, name: String) -> eframe::Result<()> {

   let v: Vec<Vec<String>> = file::read_file(name)?;
   let m: CommandMap = common::make_map(v.clone());

   let mut state = CanvasState::default();

   state.overwrite_from_map(&m);
   //println!("{:?}", state);
    
   let is_run_mode = mode == "run";
   println!("Mode: {}", is_run_mode);

   let options = eframe::NativeOptions {
       viewport: egui::ViewportBuilder::default()
       .with_inner_size([state.width, state.height])
       .with_visible(!is_run_mode)
       .with_active(!is_run_mode),
       ..Default::default()
    };

   let mode_str = mode.to_string();
   let output_path = format!("{}.{}", state.file_name, state.format);

   eframe::run_native(
       "Survival Painter",
       options,
       Box::new(|cc| {
           setup_custom_fonts(&cc.egui_ctx);
           let app = SpcApp::with_data(m, v, &mode_str, output_path);
           Ok(Box::new(app))
      })
   )
}
