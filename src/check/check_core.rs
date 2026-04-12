// src/check/check_core.rs

use crate::error::AppError;
use crate::CommandMap;
use crate::utils::file;
use crate::utils::common;
use crate::check::check_arc;
use crate::check::check_circle;
use crate::check::check_ellipse;
use crate::check::check_generic;
use crate::check::check_group;
use crate::check::check_line;
use crate::check::check_place;
use crate::check::check_polygon;
use crate::check::check_rect;
use crate::check::check_repeat;
use crate::check::check_replace;
use crate::check::check_rotation;

pub fn main(name: String) -> Result<(), AppError> {
    println!("Check Core... {}", name);

    let v: Vec<Vec<String>> = file::read_file(name)?;
    let m: CommandMap = common::make_map(v);

    let settings_tags = vec!["VN", "FN", "FT", "FW", "FH", "LC", "LW", "BG"];
    
    for tag in settings_tags {
        if m.contains_key(tag) {
            check_generic::settings_core(tag.to_string(), &m)?;
        }
    }

    let n: i32 = check_generic::get_ver("VN".to_string(), &m)?;

    check_generic::check_ops(n, &m);
    //println!("{}", n);
    check_replace::replace_core("RC".to_string(), &m)?;
    check_line::line_core(n, "LN".to_string(), &m)?;
    check_circle::circle_core(n, "CI".to_string(), &m)?;
    check_arc::arc_core(n, "AR".to_string(), &m)?;

    check_rect::rect_core(n, "RE".to_string(), &m)?;
    check_ellipse::ellipse_core(n, "EC".to_string(), &m)?;
    check_polygon::polygon_core(n, "PG".to_string(), &m)?;

    check_place::place_core(n, "PL".to_string(), &m)?;
    check_repeat::repeat_core("RP".to_string(), &m)?;

    check_group::group_core(&m)?;
    check_rotation::rotation_core(&m)?;

    Ok(())
}
