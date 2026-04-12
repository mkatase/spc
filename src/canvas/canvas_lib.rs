// src/canvas/canvas_lib.rs

use std::collections::HashMap;
use crate::CommandMap;
use crate::canvas::{canvas_circle, canvas_line, canvas_place, canvas_rotation};
use crate::canvas::{canvas_arc, canvas_ellipse, canvas_rect, canvas_group};
use crate::canvas::{canvas_repeat, canvas_polygon, canvas_text};

pub const FONT_DATA: &[u8] = include_bytes!("../../assets/Noto_Sans/static/NotoSans-Regular.ttf");

#[derive(Debug)]
pub struct CanvasState {
    pub file_name: String,  // FN (File Name)
    pub format: String,     // FT (File Type)
    pub width: f32,         // FW (File Width)
    pub height: f32,        // FH (File Height)
    pub bg_color: String,   // BG (Back Ground Color)
    pub custom_colors: HashMap<String, [u8; 4]>,  // RC(Replacement Color)
    pub groups: HashMap<String, SpcGroup>,
    pub current_gr_id: Option<String>,
    // for RO
    pub rotations: HashMap<String, SpcRotation>,
    pub current_ro_id: Option<String>,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            file_name: "spc".into(),
            format: "webp".into(),
            width: 400.0,
            height: 400.0,
            bg_color: "white".into(),
            custom_colors: HashMap::new(),
            groups: HashMap::new(),
            current_gr_id: None,
            rotations: HashMap::new(),
            current_ro_id: None,
        }
    }
}

impl CanvasState {
    pub fn overwrite_from_map(&mut self, m: &CommandMap) {
        // for FN
        if let Some(n) = m.get("FN")
            .and_then(|rows| rows.last())
            .and_then(|row| row.get(1)) { self.file_name = n.clone(); }
        // for FT
        if let Some(t) = m.get("FT")
            .and_then(|rows| rows.last())
            .and_then(|row| row.get(1)) { self.format = t.clone(); }
        // for FW
        if let Some(w) = m.get("FW")
            .and_then(|rows| rows.last())
            .and_then(|row| row.get(1))
            .and_then(|s| s.parse::<f32>().ok()) { self.width = w; }
        // for FH
        if let Some(h) = m.get("FH")
            .and_then(|rows| rows.last())
            .and_then(|row| row.get(1))
            .and_then(|s: &String| s.parse::<f32>().ok()) { self.height = h; }
        // for BG
        if let Some(g) = m.get("BG")
            .and_then(|rows| rows.last())
            .and_then(|row| row.get(1)) { self.bg_color = g.clone(); }
        // for RC
        if let Some(rows) = m.get("RC") {
            for row in rows {
                if row.len() >= 3 {
                    let name = &row[1];
                    let hex = &row[2];
                    
                    if let Ok(clean_name) = crate::canvas::canvas_color::validate_color_name(name) {
                        let rgba = crate::canvas::canvas_color::hex_to_rgba(hex);
                        self.custom_colors.insert(clean_name, rgba);
                    }
                }
            }
        }
    }
}

pub struct DrawContext {
    pub line_width: f32,      // LW (Line Width)
    pub line_color: String,   // LC (Line Color)
    pub font_size: f32,       // ?? (Font Size)
}

impl Default for DrawContext {
    fn default() -> Self {
        Self {
            line_width: 1.0,
            line_color: "black".into(),
            font_size: 16.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcGroup {
    pub commands: Vec<SpcCommand>,  // Tags in this Scope(GR)
}

#[derive(Debug, Clone)]
pub struct SpcRotation {
    pub angle: f32,                 // 45 of RO,@R1,45
    pub commands: Vec<SpcCommand>,  // Tags in this Scope(RO)
}

#[derive(Debug, Clone)]
pub struct SpcLine {
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub color: Option<String>,
    pub width: Option<f32>,
}

impl Default for SpcLine {
    fn default() -> Self {
        Self {
            p1: [0.0, 0.0],
            p2: [0.0, 0.0],
            color: None,
            width: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcCircle {
    pub center: [f32; 2],
    pub radius: f32,
    pub color: Option<String>,
    pub width: Option<f32>,
    pub fill_color: Option<String>,
    //pub is_filled: bool,
}

impl Default for SpcCircle {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            radius: 10.0,
            color: None,
            width: None,
            //is_filled: false,
            fill_color: None,
        }
    }
}

// AR (Arc/Sector)
#[derive(Debug, Clone)]
pub struct SpcArc {
    pub center: [f32; 2],
    pub radius: f32,
    pub start_deg: f32,
    pub end_deg: f32,
    pub color: Option<String>,
    pub width: Option<f32>,
    pub fill_color: Option<String>,
}

impl Default for SpcArc {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            radius: 10.0,
            start_deg: 0.0,
            end_deg: 360.0,
            color: None,
            width: None,
            fill_color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Option<String>,
    pub width: Option<f32>,
    pub fill_color: Option<String>,
}

impl Default for SpcRect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 10.0,
            color: None,
            width: None,
            fill_color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcPolygon {
    pub center: [f32; 2],
    pub r: f32,
    pub v: i32,
    pub o: f32,
    pub color: Option<String>,
    pub width: Option<f32>,
    pub fill_color: Option<String>,
}

impl Default for SpcPolygon {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            r: 10.0,
            v: 3,
            o: 0.0,
            color: None,
            width: None,
            fill_color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcEllipse {
    pub center: [f32; 2],
    pub rx: f32,
    pub ry: f32,
    pub color: Option<String>,
    pub width: Option<f32>,
    pub fill_color: Option<String>,
}

impl Default for SpcEllipse {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            rx: 10.0,
            ry: 10.0,
            color: None,
            width: None,
            fill_color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcText {
    pub pos: [f32; 2],          
    pub text: String,
    pub size: Option<f32>,
    pub color: Option<String>,
    pub angle: f32,
}

impl Default for SpcText {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0],
            text: "".into(),
            size: None,
            color: None,
            angle: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcPlace {
    pub group_id: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct SpcRepeat {
    pub group_id: String, // @G1 などのID
    pub count_x: usize, 
    pub count_y: usize,
    pub base_x: f32,
    pub base_y: f32,
    pub step_x: f32,
    pub step_y: f32,
}

#[derive(Debug, Clone)]
pub enum SpcCommand {
    Line(SpcLine),
    Circle(SpcCircle),
    Arc(SpcArc),
    Rect(SpcRect),
    Polygon(SpcPolygon),
    Ellipse(SpcEllipse),
    Text(SpcText),
    // Status Control
    LineColor(String),           // LC
    LineWidth(f32),              // LW
    FontSize(f32),               // TS
    // Block Control
    GroupStart(String),         // GR, @ID
    GroupEnd,                   // GR, E
    RotationStart(String, f32), // RO, @ID, angle
    RotationEnd,                // RO, E
    // Placement and Repeat
    Place(SpcPlace),           // PL
    Repeat(SpcRepeat),         // RP
}

impl SpcCommand {
    pub fn parse(row: &[String]) -> Option<Self> {
        if row.is_empty() { return None; }
        let tag = &row[0];
        match tag.as_str() {
            "LC" => {
                Some(SpcCommand::LineColor(row.get(1).cloned().unwrap_or_default()))
            },
            "LW" => row.get(1)
                .and_then(|s| s.parse().ok()).map(SpcCommand::LineWidth),
            "TS" => row.get(1)
                .and_then(|s| s.parse().ok()).map(SpcCommand::FontSize),
            "LN" => Some(canvas_line::calc(row)),
            "CI" => Some(canvas_circle::calc(row)),
            "RE" => Some(canvas_rect::calc(row)),
            "PG" => Some(canvas_polygon::calc(row)),
            "AR" => Some(canvas_arc::calc(row)),
            "EC" => Some(canvas_ellipse::calc(row)),
            "TE" => Some(canvas_text::calc(row)),
            "GR" => Some(canvas_group::calc(row)),
            "RO" => Some(canvas_rotation::calc(row)),
            "PL" => Some(canvas_place::calc(row)),
            "RP" => Some(canvas_repeat::calc(row)),
            // ...
            _ => None,
        }
    }
}
