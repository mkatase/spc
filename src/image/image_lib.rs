// src/image/image_lib.rs

use ab_glyph::FontArc;
use imageproc::drawing;
use imageproc::rect::Rect;
use image::{Rgba, RgbaImage};
use crate::canvas::canvas_lib::{SpcLine, SpcCircle, SpcText};
use crate::canvas::canvas_lib::{SpcArc, SpcEllipse, SpcRect, SpcPolygon};
use crate::canvas::canvas_exec::SpcExecutor;
use crate::canvas::canvas_lib::{CanvasState, DrawContext, FONT_DATA};
use crate::canvas::canvas_calc;

impl SpcExecutor for RgbaImage {
    fn draw_line(&mut self, data: SpcLine,
        state: &CanvasState, ctx: &DrawContext) {
        let (sraw, eraw, stroke_rgba, _w0) = data.resolve(state, ctx);
        let sp = (sraw[0], sraw[1]);
        let ep = (eraw[0], eraw[1]);
        drawing::draw_line_segment_mut(self, sp, ep, Rgba(stroke_rgba));
    }

    fn draw_circle(&mut self, data: SpcCircle,
        state: &CanvasState, ctx: &DrawContext) {
        let (ce, ra, stroke_rgba, _w0, fill_rgba) = data.resolve(state, ctx);
        let cp = (ce[0] as i32, ce[1] as i32);
        let ra = ra as i32;

        if fill_rgba[3] > 0 {
            drawing::draw_filled_circle_mut(self, cp, ra, Rgba(fill_rgba));
        }

        drawing::draw_hollow_circle_mut(self, cp, ra, Rgba(stroke_rgba));
    }

    fn draw_arc(&mut self, data: SpcArc,
        state: &CanvasState, ctx: &DrawContext) {
        let (cce, ra, ssd, eed, stroke_rgba, _w0, _fill_rgba) = data.resolve(state, ctx);
        let points
            = canvas_calc::arc_points(cce, ra, ssd, eed, 1.0);

        for p in points {
            let x = p[0].round() as i32;
            let y = p[1].round() as i32;
        
            if x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32 {
                self.put_pixel(x as u32, y as u32, Rgba(stroke_rgba));
            }
        }
    }

    fn draw_ellipse(&mut self, data: SpcEllipse,
        state: &CanvasState, ctx: &DrawContext) {
        let (cce, rra, stroke_rgba, _w0, fill_rgba) = data.resolve(state, ctx);
        let ce = (cce[0] as i32, cce[1] as i32);
        let wr = rra[0] as i32; // width radius
        let hr = rra[1] as i32; // height radius
                                
        if fill_rgba[3] > 0 {
            drawing::draw_filled_ellipse_mut( self, ce, wr, hr, Rgba(fill_rgba) );
        }
        drawing::draw_hollow_ellipse_mut( self, ce, wr, hr, Rgba(stroke_rgba) );
    }

    fn draw_rect(&mut self, data: SpcRect,
        state: &CanvasState, ctx: &DrawContext) {
        let (tl, br, stroke_rgba, _w0, fill_rgba) = data.resolve(state, ctx);
        let x = tl[0] as i32;
        let y = tl[1] as i32;
        let w = (br[0] as i32 - x) as u32;
        let h = (br[1] as i32 - y) as u32;
        let r = Rect::at(x, y).of_size(w, h);

        if fill_rgba[3] > 0 {
            drawing::draw_filled_rect_mut( self, r, Rgba(fill_rgba) );
        }
        drawing::draw_hollow_rect_mut( self, r, Rgba(stroke_rgba) );
    }

    fn draw_polygon(&mut self, data: SpcPolygon,
        state: &CanvasState, ctx: &DrawContext) {
    
        let (cce, r, v, o, stroke_rgba, _w0, fill_rgba) = data.resolve(state, ctx);
        let coords = canvas_calc::poly_points(cce, r, v, o);
    
        let i_points: Vec<imageproc::point::Point<i32>> = 
            coords.iter().map(|&[x, y]| {
                imageproc::point::Point::new(x as i32, y as i32)
            })
            .collect();
        let f_points: Vec<imageproc::point::Point<f32>> = 
            coords.iter().map(|&[x, y]| {
                imageproc::point::Point::new(x, y)
            })
            .collect();

        if fill_rgba[3] > 0 {
            imageproc::drawing::draw_polygon_mut(self, &i_points, Rgba(fill_rgba));
        }
        imageproc::drawing::draw_hollow_polygon_mut(self, &f_points, Rgba(stroke_rgba));
    }

    fn draw_text(&mut self, data: SpcText,
        state: &CanvasState, ctx: &DrawContext) {
        let (pos, color_rgba, size) = data.resolve(state, ctx);

        let font = FontArc::try_from_slice(FONT_DATA)
            .expect("Error loading font");

        imageproc::drawing::draw_text_mut(
            self,
            Rgba(color_rgba),
            pos[0] as i32,
            pos[1] as i32,
            size,
            &font,
            &data.text,
        );
    }
}
