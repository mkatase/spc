// src/check/check_lib.rs

use std::collections::{HashMap, HashSet};

pub const LATEST_VERSION: i32 = 1;
pub type ReplaceMap = HashMap<String, String>;


pub enum ParamType {
    Float, 
    Int,
    Str,
    GroupId,
    Keyword,
    Color,
    Any,
}

use ParamType::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType {
    VN,                            // Version
    FN, FT, FW, FH, BG, TS,        // CanvasState
    LW, LC,                        // DrawContext
    RC, LN, CI, EC, AR, RE, OF,    // V1
    GR, PL, RO, RP, PG, TE,        // V1
    LT, VE,                        // V2
}

impl CommandType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "VN" => Some(Self::VN),
            "FN" => Some(Self::FN),
            "FT" => Some(Self::FT),
            "FW" => Some(Self::FW),
            "FH" => Some(Self::FH),
            "BG" => Some(Self::BG),
            "LW" => Some(Self::LW),
            "LC" => Some(Self::LC),
            "TS" => Some(Self::TS),
            "RC" => Some(Self::RC),
            "LN" => Some(Self::LN),
            "LT" => Some(Self::LT),
            "CI" => Some(Self::CI),
            "EC" => Some(Self::EC),
            "AR" => Some(Self::AR),
            "RE" => Some(Self::RE),
            "TE" => Some(Self::TE),
            "GR" => Some(Self::GR),
            "RO" => Some(Self::RO),
            "PL" => Some(Self::PL),
            "RP" => Some(Self::RP),
            "PG" => Some(Self::PG),
            "OF" => Some(Self::OF),
            "VE" => Some(Self::VE),
            _ => None,
        }
    }

    pub fn spec(&self) -> &'static [ParamType] {
        match self {
            // Canvas/Context: Tag, Value
            Self::VN | Self::FN | Self::FT | Self::TS | 
            Self::FW | Self::FH | Self::BG | Self::LW | Self::LC 
                => &[Str, Any],

            // LN: Tag, x1, y1, x2, y2, [color, width]
            Self::LN => &[Str, Float, Float, Float, Float, Color, Float],

            // CI: Tag, x, y, r, [color, width, fill_color]
            Self::CI => &[Str, Float, Float, Float, Color, Float, Color],

            // EC/RE: Tag, x, y, w, h, [color, width, fill_color]
            Self::EC | Self::RE => &[Str, Float, Float, Float, Float, Color, Float, Color],

            // AR: Tag, x, y, r, start, end, [color, width, fill_color]
            Self::AR => &[Str, Float, Float, Float, Float, Float, Color, Float, Color],

            // PG: Tag, x, y, r, vertices, angle, [color, width, fill_color]
            Self::PG => &[Str, Float, Float, Float, Int, Float, Color, Float, Color],

            // TE: Tag, x, y, text, [size, color]
            Self::TE => &[Str, Float, Float, Str, Float, Color],

            // PL: Tag, @ID, x, y, [rotation, scale]
            Self::PL => &[Str, Str, Float, Float, Float, Float],

            // RP: Tag, @ID, count_x, count_y, base_x, base_y, step_x, step_y
            Self::RP => &[Str, Str, Int, Int, Float, Float, Float, Float],

            // GR/RO: Tag, @ID (or "E")
            Self::GR | Self::RO => &[Str, Str],

            _ => &[Str],
        }
    }

    pub fn expected_range(&self, ver: i32) ->
        std::ops::RangeInclusive<usize> {
        match (self, ver) {
            // VN, n
            (Self::VN, 1) => 2..=2,
            // FN, name ; FT, type ; FW, f32 ; FH, f32
            // BG, str ;
            (Self::FN, 1) | (Self::FT, 1) | (Self::TS, 1) |
            (Self::FW, 1) | (Self::FH, 1) | (Self::BG, 1)
                => 2..=2,
            // LW, f32 ; LC, str;
            (Self::LW, 1) | (Self::LC, 1) => 2..=2,
            // LN, x1, y1, x2, y2 [,color, style ]
            (Self::LN, 1) => 5..=7,        // Tag(1) + 4..6
            // CI, x, y, r [, color, fill]
            (Self::CI, 1) => 4..=6,        // Tag(1) + 3..5
            // EC, x, y, w, h, [, color, fill]
            (Self::EC, 1) => 5..=7,        // Tag(1) + 4..6
            // AR, x, y, r, s, e, [, color, fill]
            (Self::AR, 1) => 6..=8,        // Tag(1) + 5..7
            // RE, x, y, w, h, [, color, fill]
            (Self::RE, 1) => 5..=7,        // Tag(1) + 4..6
            // PG, x, y, r, v, o, [, color, fill]
            (Self::PG, 1) => 6..=8,        // Tag(1) + 5..7
            // TE, x, y, t [, size, color]
            (Self::TE, 1) => 4..=6,        // Tag(1) + 3..5
            // GR, @ID (start) or GR, E (end)
            (Self::GR, 1) => 2..=2,
            // RO, @ID (start) or RO, E (end)
            (Self::RO, 1) => 2..=2,
            // PL, @ID, x, y [, rotation, scale]
            (Self::PL, 1) => 4..=6,
            // RP, @ID, count_x, count_y, base_x, base_y, step_x, step_y
            (Self::RP, 1) => 8..=8,
            _ => 1..=255,
        }
    }
}

pub struct Checker {
    pub version: i32,
    pub allowed_commands: HashSet<CommandType>,
}

impl Checker {
    pub fn new(version: i32) -> Self {
        let mut allowed = HashSet::new();
        match version {
            1 => {
                // Version 1
                let cmds = vec![
                    CommandType::VN,CommandType::FN,
                    CommandType::FT,CommandType::FW,
                    CommandType::FH,CommandType::BG,
                    CommandType::LW,CommandType::LC,
                    CommandType::RC,CommandType::LN,
                    CommandType::CI,CommandType::EC,
                    CommandType::AR,CommandType::RO,
                    CommandType::GR,CommandType::PL,
                    CommandType::RE,CommandType::RP,
                    CommandType::PG,CommandType::TS,
                    CommandType::TE,
                ];
                for c in cmds { allowed.insert(c); }
            }
            2 => {
                // Version 2
                let cmds = vec![
                    CommandType::VN,CommandType::FN,
                    CommandType::FT,CommandType::FW,
                    CommandType::FH,CommandType::BG,
                    CommandType::LW,CommandType::LC,
                    CommandType::RC,CommandType::LN,
                    CommandType::CI,CommandType::EC,
                    CommandType::AR,CommandType::RO,
                    CommandType::GR,CommandType::PL,
                    CommandType::RE,CommandType::RP,
                    CommandType::PG,CommandType::OF,
                    CommandType::LT,CommandType::VE,
                    CommandType::TS,CommandType::TE,
                ];
                for c in cmds { allowed.insert(c); }
            }
            _ => {} // Unknown Version
        }
        Self { version, allowed_commands: allowed }
    }

    pub fn is_allowed(&self, cmd: CommandType) -> bool {
        self.allowed_commands.contains(&cmd)
    }
}
