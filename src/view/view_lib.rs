// src/view/view_lib.rs

use eframe::egui;
use eframe::egui::epaint::PathStroke;
use eframe::egui::epaint::EllipseShape;
use eframe::egui::epaint::Vec2;

use crate::CommandMap;
use crate::utils::file;
use crate::canvas::canvas_lib;
use crate::canvas::canvas_exec::SpcExecutor;
use crate::canvas::canvas_calc;
use crate::view::view_core;

pub struct SpcApp {
    pub map: CommandMap,
    pub rows: Vec<Vec<String>>,
    pub mode: String,         // view or run
    pub output_path: String,
    pub is_captured: bool,   // protect flag
}

impl SpcApp {
    pub fn with_data(m: CommandMap, rows: Vec<Vec<String>>, mode: &str, path: String) -> Self {
        Self {
            map: m,
            rows: rows,
            mode: mode.to_string(),
            output_path: path,
            is_captured: false,
        }
    }
}

impl eframe::App for SpcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut exit_req = false;

        if self.mode == "run" && self.is_captured {
            ctx.request_repaint();
        } else {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::Escape) {
                exit_req = true;
            }
        });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            view_core::draw(self, ui);
        });

        if self.mode == "run" && !self.is_captured {
            ctx.send_viewport_cmd(
                egui::ViewportCommand::Screenshot(egui::UserData::default()));
            self.is_captured = true;
        }
            ctx.input(|i| {
                for event in &i.raw.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        file::save_image(image, &self.output_path);
                        exit_req = true;
                        println!("Close...");
                    }
                }
            });

        if exit_req {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl SpcExecutor for egui::Painter {
    fn draw_line(&mut self, data: canvas_lib::SpcLine, 
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        let (pp1, pp2, stroke_rgba, w0) = data.resolve(state, ctx);
        // println!(">>> [DEBUG LINE] h:{} | p1:{:?}, p2:{:?} | color:{:?}", state.height, pp1, pp2, stroke_rgba);
        let p1 = egui::pos2(pp1[0], pp1[1]);
        let p2 = egui::pos2(pp2[0], pp2[1]);

        let stroke_color = egui::Color32::from_rgba_unmultiplied(
            stroke_rgba[0], stroke_rgba[1], stroke_rgba[2], stroke_rgba[3]
        );
        let st = egui::Stroke{width: w0, color:stroke_color};
        self.line_segment([p1, p2], st);
    }

    fn draw_circle(&mut self, data: canvas_lib::SpcCircle, 
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        let (cce, ra, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        //println!(">>> [DEBUG CIRCLE] center:{:?}, radius:{} | color:{} | bool:{}", cce, ra, c0, b0);
        //println!(">>> [DEBUG CIRCLE] h:{} | center:{:?}, radius:{} | color:{}", state.height, cce, ra, c0);
        let ce = egui::pos2(cce[0], cce[1]);

        let stroke_color = egui::Color32::from_rgba_unmultiplied(
            stroke_rgba[0], stroke_rgba[1], stroke_rgba[2], stroke_rgba[3]
        );
        let st = egui::Stroke{width: w0, color: stroke_color};

        if fill_rgba[3] > 0 {
            let fill_color = egui::Color32::from_rgba_unmultiplied(
                fill_rgba[0], fill_rgba[1], fill_rgba[2], fill_rgba[3]
            );
            self.circle_filled(ce, ra, fill_color);
        }
    
        self.circle_stroke(ce, ra, st);
    }

    fn draw_arc(&mut self, data: canvas_lib::SpcArc, 
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        let (cce, ra, ssd, eed, stroke_rgba, w0, _fill_rgba) = data.resolve(state, ctx);
        let coords = canvas_calc::arc_points(cce, ra, ssd, eed, 0.1);
        let points: Vec<egui::Pos2> = coords.into_iter()
            .map(|[x, y]| egui::pos2(x, y))
            .collect();

        let stroke_color = egui::Color32::from_rgba_unmultiplied(
            stroke_rgba[0], stroke_rgba[1], stroke_rgba[2], stroke_rgba[3]
        );
        
        let st = PathStroke{
            width: w0,
            color: eframe::epaint::ColorMode::Solid(stroke_color),
            kind: egui::StrokeKind::Outside,
        };
        let shape = egui::Shape::Path(egui::epaint::PathShape {
            points,
            closed: false, // true is Pie
            fill: egui::Color32::TRANSPARENT,
            stroke: st,
        });
        self.add(shape);
        self.circle_filled(cce.into(), 2.0, stroke_color);
    }

    fn draw_ellipse(&mut self, data: canvas_lib::SpcEllipse, 
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        let (cce, ra, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        //println!(">>> [DEBUG ELLIPSE] cen:{:?} | radius:{:?}, color:{:?}", cce, ra, stroke_rgba);
        let pp = egui::pos2(cce[0], cce[1]);
        let rr = Vec2{x: ra[0], y: ra[1]};
        let stroke_color = egui::Color32::from_rgba_unmultiplied(
            stroke_rgba[0], stroke_rgba[1], stroke_rgba[2], stroke_rgba[3]
        );
        let st = egui::Stroke{width: w0, color:stroke_color};
        // for Color32
        let fill_color = egui::Color32::from_rgba_unmultiplied(
            fill_rgba[0], fill_rgba[1], fill_rgba[2], fill_rgba[3]
        );

        let shape = EllipseShape {
            center: pp,
            radius: rr,
            fill: fill_color,
            stroke: st,
        };
        self.add(shape);
    }

    fn draw_rect(&mut self, data: canvas_lib::SpcRect, 
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        //println!("--- draw_rect in view_lib.rs ---");
        let (pp1, pp2, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        //println!(">>> [DEBUG RECT] min:{:?} | max:{:?}, color:{:?}", pp1, pp2, stroke_rgba);
        let p1 = egui::pos2(pp1[0], pp1[1]);
        let p2 = egui::pos2(pp2[0], pp2[1]);
        let rec = egui::Rect{min: p1, max: p2};
        let cr = egui::CornerRadius{nw:0 , ne:0, sw:0, se:0};
        let sk = egui::StrokeKind::Outside;
        //let co = canvas_color::get_color(&c0);
        let stroke_color = egui::Color32::from_rgba_unmultiplied(
            stroke_rgba[0], stroke_rgba[1], stroke_rgba[2], stroke_rgba[3]
        );
        let st = egui::Stroke{width: w0, color:stroke_color};

        if fill_rgba[3] > 0 {
            println!("--- rect is filled ---");
            let fill_color = egui::Color32::from_rgba_unmultiplied(
                fill_rgba[0], fill_rgba[1], fill_rgba[2], fill_rgba[3]
            );
            self.rect_filled(rec, cr, fill_color);
        }
        println!("--- rect is not filled ---");
        self.rect_stroke(rec, cr, st, sk);
    }

    fn draw_polygon(&mut self, data: canvas_lib::SpcPolygon,
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        //println!("--- draw_polygon in view_lib.rs ---");
        let (cce, r, v, o, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        let coords = canvas_calc::poly_points(cce, r, v, o);
        let points: Vec<egui::Pos2> = coords.into_iter()
            .map(|[x, y]| egui::pos2(x, y))
            .collect();

        let stroke_color = egui::Color32::from_rgba_unmultiplied(
            stroke_rgba[0], stroke_rgba[1], stroke_rgba[2], stroke_rgba[3]
        );
        let st = egui::Stroke{width: w0, color:stroke_color};

        let fill_color = egui::Color32::from_rgba_unmultiplied(
            fill_rgba[0], fill_rgba[1], fill_rgba[2], fill_rgba[3]
        );

        let shape = egui::Shape::convex_polygon(
            points,
            fill_color,
            st
        );
        self.add(shape);
    }

    fn draw_text(&mut self, data: canvas_lib::SpcText,
        state: &canvas_lib::CanvasState, ctx: &canvas_lib::DrawContext) {
        //println!("--- draw_text in view_lib.rs ---");

        let (pos, color_rgba, size) = data.resolve(state, ctx);
        let color = egui::Color32::from_rgba_unmultiplied(
            color_rgba[0], color_rgba[1], color_rgba[2], color_rgba[3]);
    
        // default font
        self.text(
            egui::pos2(pos[0], pos[1]),
            egui::Align2::LEFT_TOP,
            &data.text,
            egui::FontId::proportional(size),
            color,
        );
    }
}
