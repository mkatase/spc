// src/canvas/canvas_calc.rs
//use eframe::egui;
use std::f32::consts::PI;
use crate::canvas::canvas_lib::{SpcCommand, SpcPlace};

pub fn apply_place(cmd: &mut SpcCommand, place: &SpcPlace, angle: f32) {
    match cmd {
        // --- Line ---
        SpcCommand::Line(l) => {
            let (p1x, p1y) = transform_single(l.p1[0], l.p1[1], place, angle);
            let (p2x, p2y) = transform_single(l.p2[0], l.p2[1], place, angle);
            l.p1 = [p1x, p1y];
            l.p2 = [p2x, p2y];
        },
        // --- Circle ---
        SpcCommand::Circle(c) => {
            let (cx, cy) = transform_single(c.center[0], c.center[1], place, angle);
            c.center = [cx, cy];
        },
        // --- Ellipse ---
        SpcCommand::Ellipse(e) => {
            let (cx, cy) = transform_single(e.center[0], e.center[1], place, angle);
            e.center = [cx, cy];
        },
        // --- Rectangle ---
        SpcCommand::Rect(r) => {
            let (x1, y1) = transform_single(r.x, r.y, place, angle);
            r.x = x1;
            r.y = y1;
        },
        // --- Arc ---
        SpcCommand::Arc(a) => {
            let (cx, cy) = transform_single(a.center[0], a.center[1], place, angle);
            a.center = [cx, cy];
            let angle_rad = angle.to_radians();
            a.start_deg += angle_rad;
            a.end_deg += angle_rad;
        },
        // --- Polygon ---
        SpcCommand::Polygon(p) => {
            //println!(">>> [CALC] Before: {:?} | angle: {}", p.center, angle);
            let (cx, cy) = transform_single(p.center[0], p.center[1], place, angle);
            p.center = [cx, cy];
            //println!(">>> [CALC] After:  {:?}", p.center);
            p.o += angle;
        },
        // --- Reserve ---
        _ => {}
    }
}

fn transform_single(x: f32, y: f32, place: &SpcPlace, angle: f32) -> (f32, f32) {
    let (rx, ry) = rotate(x, y, angle);
    translate(rx, ry, place.x, place.y)
}

pub fn rotate(x: f32, y: f32, angle_deg: f32) -> (f32, f32) {
    let rad = angle_deg.to_radians();
    let nx = x * rad.cos() - y * rad.sin();
    let ny = x * rad.sin() + y * rad.cos();
    (nx, ny)
}

pub fn translate(x: f32, y: f32, dx: f32, dy: f32) -> (f32, f32) {
    (x + dx, y + dy)
}

pub fn to_device_coords(x: f32, y: f32, canvas_height: f32) -> (f32, f32) {
    (x, canvas_height - y)
}

pub fn arc_points(ce: [f32; 2], ra: f32, sd: f32, ed: f32, st: f32) -> Vec<[f32; 2]> {
    let mut points = Vec::new();
    let step  = st;
    let mut d = sd;
    
    while d <= ed {
        let rad = d.to_radians();
        points.push([
            ce[0] + ra * rad.cos(),
            ce[1] - ra * rad.sin()
        ]);
        d += step;
    }
    
    let rad_end = ed.to_radians();
    points.push([ce[0] + ra * rad_end.cos(), ce[1] - ra * rad_end.sin()]);

    points
}

pub fn poly_points(ce: [f32; 2], r: f32, v: i32, o: f32) -> Vec<[f32; 2]> {
    let mut points = Vec::new();
    let offset_rad = o.to_radians();
    for i in 0..v {
        let rad = ( 2 as f32 * PI * i as f32) / v  as f32 + offset_rad;
        points.push([
            ce[0] + r * rad.cos(),
            ce[1] - r * rad.sin()
        ])
    }
    points
}
