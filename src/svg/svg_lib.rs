// src/svg/svg_lib.rs

use crate::canvas::canvas_exec::SpcExecutor;
use crate::canvas::canvas_lib::{SpcLine, SpcCircle, CanvasState, DrawContext};
use crate::canvas::canvas_lib::{SpcArc, SpcEllipse, SpcRect, SpcPolygon, SpcText};
use crate::canvas::{canvas_calc, canvas_color};

pub struct SvgBackend {
    pub shapes: Vec<String>,
    pub width: f32,
    pub height: f32,
}

impl SvgBackend {
    pub fn new(w: f32, h: f32) -> Self {
        Self { shapes: Vec::new(), width: w, height: h }
    }

    pub fn finalize(&self, bg_color: &str) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            self.width, self.height, self.width, self.height
        );
        svg.push_str("\n");
        svg.push_str(&format!(r#"  <rect width="100%" height="100%" fill="{}" />"#, bg_color));
        svg.push_str("\n");
        svg.push_str(&self.shapes.join("\n"));
        svg.push_str("\n</svg>");
        svg
    }
}

impl SpcExecutor for SvgBackend {
    fn draw_line(&mut self, data: SpcLine, state: &CanvasState, ctx: &DrawContext) {
        let (s, e, stroke_rgba, w0) = data.resolve(state, ctx);
        let stroke_str = canvas_color::to_svg_str(stroke_rgba);

        self.shapes.push(format!(
            r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
            s[0], s[1], e[0], e[1], stroke_str, w0
        ));
    }

   fn draw_circle(&mut self, data: SpcCircle,
        state: &CanvasState, ctx: &DrawContext) {
        let (ce, ra, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);

        let fill_str = canvas_color::to_svg_str(fill_rgba);
        let stroke_str = canvas_color::to_svg_str(stroke_rgba);

        self.shapes.push(format!(
            r#"  <circle cx="{}" cy="{}" r="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
            ce[0], ce[1], ra, fill_str, stroke_str, w0
        ));
    }

    fn draw_arc(&mut self, data: SpcArc, state: &CanvasState, ctx: &DrawContext) {
        let (cce, ra, ssd, eed, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        let points = canvas_calc::arc_points(cce, ra, ssd, eed, 1.0);

        let points_attr = points.iter()
            .map(|p| format!("{:.2},{:.2}", p[0], p[1]))
            .collect::<Vec<_>>()
            .join(" ");

        let fill_str = canvas_color::to_svg_str(fill_rgba);
        let stroke_str = canvas_color::to_svg_str(stroke_rgba);

        self.shapes.push(format!(
            r#"  <polyline points="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
            points_attr, fill_str, stroke_str, w0
        ));
    }

    fn draw_ellipse(&mut self, data: SpcEllipse, state: &CanvasState, ctx: &DrawContext) {
        let (cce, rra, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        let fill_str = canvas_color::to_svg_str(fill_rgba);
        let stroke_str = canvas_color::to_svg_str(stroke_rgba);
    
        self.shapes.push(format!(
            r#"  <ellipse cx="{}" cy="{}" rx="{}" ry="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
            cce[0], cce[1], rra[0], rra[1], fill_str, stroke_str, w0
        ));
    }

    fn draw_rect(&mut self, data: SpcRect, state: &CanvasState, ctx: &DrawContext) {
        let (tl, br, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        let w = br[0] - tl[0];
        let h = br[1] - tl[1];

        let fill_str = canvas_color::to_svg_str(fill_rgba);
        let stroke_str = canvas_color::to_svg_str(stroke_rgba);

        self.shapes.push(format!(
            r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
            tl[0], tl[1], w, h, fill_str, stroke_str, w0
        ));
    }

    fn draw_polygon(&mut self, data: SpcPolygon, state: &CanvasState, ctx: &DrawContext) {
        let (cce, r, v, o, stroke_rgba, w0, fill_rgba) = data.resolve(state, ctx);
        let coords = canvas_calc::poly_points(cce, r, v, o); // 🚩 共通演算コア使用

        let points_attr = coords.iter()
            .map(|p| format!("{:.2},{:.2}", p[0], p[1]))
            .collect::<Vec<_>>()
            .join(" ");

        let fill_str = canvas_color::to_svg_str(fill_rgba);
        let stroke_str = canvas_color::to_svg_str(stroke_rgba);

        self.shapes.push(format!(
            r#"  <polygon points="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
            points_attr, fill_str, stroke_str, w0
        ));
    }

    fn draw_text(&mut self, data: SpcText, state: &CanvasState, ctx: &DrawContext) {
        let (pos, color_rgba, size) = data.resolve(state, ctx);
        let color_str = canvas_color::to_svg_str(color_rgba);
        if data.angle != 0.0 {
            self.shapes.push(format!(
                r#"  <text x="{}" y="{}" font-family="Noto Sans, sans-serif" font-size="{}" fill="{}" transform="rotate({} {}, {})">{}</text>"#,
                pos[0], pos[1], size, color_str, -data.angle, pos[0], pos[1], data.text
            ));
        } else { 
            self.shapes.push(format!(
                r#"  <text x="{}" y="{}" font-family="Noto Sans, sans-serif" font-size="{}" fill="{}">{}</text>"#,
                pos[0], pos[1], size, color_str, data.text
            ));
        }
    }
}
