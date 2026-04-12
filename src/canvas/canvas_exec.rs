// src/canvas/canvas_exec.rs

use crate::canvas::canvas_lib::{SpcCommand, CanvasState, DrawContext};
use crate::canvas::canvas_lib::{SpcLine, SpcCircle, SpcRotation};
use crate::canvas::canvas_lib::{SpcArc, SpcEllipse, SpcRect, SpcPolygon};
use crate::canvas::canvas_lib::{SpcText, SpcGroup, SpcPlace};
use crate::canvas::canvas_calc;

pub trait SpcExecutor {
    fn draw_line(&mut self, line: SpcLine, state: &CanvasState, ctx: &DrawContext);
    fn draw_circle(&mut self, circle: SpcCircle, state: &CanvasState, ctx: &DrawContext);
    fn draw_arc(&mut self, arc: SpcArc, state: &CanvasState, ctx: &DrawContext);
    fn draw_ellipse(&mut self, ellipse: SpcEllipse, state: &CanvasState, ctx: &DrawContext);
    fn draw_rect(&mut self, rect: SpcRect, state: &CanvasState, ctx: &DrawContext);
    fn draw_polygon(&mut self, rect: SpcPolygon, state: &CanvasState, ctx: &DrawContext);
    fn draw_text(&mut self, rect: SpcText, state: &CanvasState, ctx: &DrawContext);

    fn execute(&mut self, cmd: SpcCommand, state: &mut CanvasState, ctx: &mut DrawContext) {
        if let Some(id) = &state.current_gr_id {
            if let SpcCommand::GroupEnd = cmd {
                state.current_gr_id = None;
            } else if let Some(gr_data) = state.groups.get_mut(id) {
                gr_data.commands.push(cmd);
            }
            return;
        }

        if let Some(id) = &state.current_ro_id {
            if let SpcCommand::RotationEnd = cmd {
                state.current_ro_id = None;
            } else if let Some(ro_data) = state.rotations.get_mut(id) {
                ro_data.commands.push(cmd);
            }
            return;
        }

        match cmd {
            SpcCommand::LineColor(c) => ctx.line_color = c,
            SpcCommand::LineWidth(w) => ctx.line_width = w,
            SpcCommand::FontSize(s)  => ctx.font_size = s,

            SpcCommand::Line(l) => self.draw_line(l, state, ctx),
            SpcCommand::Circle(c) => self.draw_circle(c, state, ctx),
            SpcCommand::Rect(r) => self.draw_rect(r, state, ctx),
            SpcCommand::Polygon(p) => self.draw_polygon(p, state, ctx),
            SpcCommand::Arc(a) => self.draw_arc(a, state, ctx),
            SpcCommand::Ellipse(e) => self.draw_ellipse(e, state, ctx),
            SpcCommand::Text(t) => self.draw_text(t, state, ctx),
            SpcCommand::GroupStart(id) => {
                state.current_gr_id = Some(id.clone());
                state.groups.insert(id, SpcGroup { commands: Vec::new() });
            },
            SpcCommand::RotationStart(id, angle) => {
                state.current_ro_id = Some(id.clone());
                state.rotations.insert(id, SpcRotation { angle, commands: Vec::new() });
            },
            SpcCommand::Place(place) => {
                // for GR
                if let Some(gr_data) = state.groups.get(&place.group_id).cloned() {
                    for mut stored_cmd in gr_data.commands {
                        canvas_calc::apply_place(&mut stored_cmd, &place, 0.0);
                        self.execute(stored_cmd, state, ctx);
                    }
                }
                // for RO
                if let Some(ro_data) = state.rotations.get(&place.group_id).cloned() {
                    for mut stored_cmd in ro_data.commands {
                        if let SpcCommand::Text(ref mut t) = stored_cmd {
                            t.angle = ro_data.angle;
                        }
                        canvas_calc::apply_place(&mut stored_cmd, &place, ro_data.angle);
                        self.execute(stored_cmd, state, ctx);
                    }
                }
            },
            SpcCommand::Repeat(rp) => {
                for iy in 0..rp.count_y {
                    for ix in 0..rp.count_x {
                        let x = rp.base_x + (rp.step_x * ix as f32);
                        let y = rp.base_y + (rp.step_y * iy as f32);

                        let virtual_pl = SpcPlace {
                            group_id: rp.group_id.clone(),
                            x,
                            y,
                        };
                        self.execute(SpcCommand::Place(virtual_pl), state, ctx);
                }
            }
        },
            _ => {}
        }
    }
}
