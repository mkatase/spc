// src/canvas/canvas_resolve.rs

use crate::canvas::canvas_lib::CanvasState;
use crate::canvas::canvas_lib::DrawContext;
use crate::canvas::canvas_lib::SpcLine;
use crate::canvas::canvas_lib::SpcCircle;
use crate::canvas::canvas_lib::SpcArc;
use crate::canvas::canvas_lib::SpcRect;
use crate::canvas::canvas_lib::SpcPolygon;
use crate::canvas::canvas_lib::SpcEllipse;
use crate::canvas::canvas_lib::SpcText;

impl SpcLine {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext)
        -> ([f32; 2], [f32; 2], [u8; 4], f32) {
        let p1 = [self.p1[0], state.height - self.p1[1]];
        let p2 = [self.p2[0], state.height - self.p2[1]];
        let width = self.width.unwrap_or(ctx.line_width);
        let color_name = self.color.as_deref().unwrap_or(&ctx.line_color);
        let rgba = crate::canvas::canvas_color::resolve_color(color_name, state);

        (p1, p2, rgba, width)
    }
}

impl SpcCircle {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext) -> 
        ([f32; 2], f32, [u8; 4], f32, [u8; 4]) 
    {
        let pos = [self.center[0], state.height - self.center[1]];

        let stroke_name = self.color.as_ref().unwrap_or(&ctx.line_color);
        let stroke_rgba = crate::canvas::canvas_color::resolve_color(stroke_name, state);

        let width = self.width.unwrap_or(ctx.line_width);

        let fill_rgba = self.fill_color.as_ref()
            .map(|c| crate::canvas::canvas_color::resolve_color(c, state))
            .unwrap_or([0, 0, 0, 0]);

        (pos, self.radius, stroke_rgba, width, fill_rgba)
    }
}

impl SpcArc {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext) -> 
        ([f32; 2], f32, f32, f32, [u8; 4], f32, [u8; 4]) {
        let pos = [self.center[0], state.height - self.center[1]];

        let stroke_name = self.color.as_ref().unwrap_or(&ctx.line_color);
        let stroke_rgba = crate::canvas::canvas_color::resolve_color(stroke_name, state);

        let width = self.width.unwrap_or(ctx.line_width);

        let fill_rgba = self.fill_color.as_ref()
            .map(|c| crate::canvas::canvas_color::resolve_color(c, state))
            .unwrap_or([0, 0, 0, 0]);
                                     
        (pos, self.radius, self.start_deg, self.end_deg, stroke_rgba, width, fill_rgba)
    }
}

impl SpcEllipse {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext) -> 
        ([f32; 2], [f32; 2], [u8; 4], f32, [u8; 4]) {
        let center = [self.center[0], state.height - self.center[1]];
        let radius = [self.rx, self.ry];

        let stroke_name = self.color.as_ref().unwrap_or(&ctx.line_color);
        let stroke_rgba = crate::canvas::canvas_color::resolve_color(stroke_name, state);

        let width = self.width.unwrap_or(ctx.line_width);

        let fill_rgba = self.fill_color.as_ref()
            .map(|c| crate::canvas::canvas_color::resolve_color(c, state))
            .unwrap_or([0, 0, 0, 0]);
        
        ( center, radius, stroke_rgba, width, fill_rgba )
    }
}

impl SpcRect {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext) -> 
        ([f32; 2], [f32; 2], [u8; 4], f32, [u8; 4]) {
        let tl = [self.x, state.height - (self.y + self.h)];
        let br = [self.x + self.w, state.height - self.y];

        let stroke_name = self.color.as_ref().unwrap_or(&ctx.line_color);
        let stroke_rgba = crate::canvas::canvas_color::resolve_color(stroke_name, state);
        let width = self.width.unwrap_or(ctx.line_width);

        let fill_rgba = self.fill_color.as_ref()
            .map(|c| crate::canvas::canvas_color::resolve_color(c, state))
            .unwrap_or([0, 0, 0, 0]);

        ( tl, br, stroke_rgba, width, fill_rgba )
    }
}

impl SpcPolygon {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext) -> 
        ([f32; 2], f32, i32, f32, [u8; 4], f32, [u8; 4]) {
        let center = [self.center[0], state.height - self.center[1]];
                                                                    
        let stroke_name = self.color.as_ref().unwrap_or(&ctx.line_color);
        let stroke_rgba = crate::canvas::canvas_color::resolve_color(stroke_name, state);
        let width = self.width.unwrap_or(ctx.line_width);

        let fill_rgba = self.fill_color.as_ref()
            .map(|c| crate::canvas::canvas_color::resolve_color(c, state))
            .unwrap_or([0, 0, 0, 0]);

        ( center, self.r, self.v, self.o, stroke_rgba, width, fill_rgba )
    }
}


impl SpcText {
    pub fn resolve(&self, state: &CanvasState, ctx: &DrawContext) -> 
        ([f32; 2], [u8; 4], f32) 
    {
        let pos = [self.pos[0], state.height - self.pos[1]];

        let stroke_name = self.color.as_ref().unwrap_or(&ctx.line_color);
        let stroke_rgba = crate::canvas::canvas_color::resolve_color(stroke_name, state);

        let size = self.size.unwrap_or(ctx.font_size);

        (pos, stroke_rgba, size)
    }
}
