// src/check/check_rect.rs

use crate::error::AppError;
use crate::CommandMap;
use crate::check::check_generic;
use crate::check::check_lib::CommandType;

pub fn rect_core(_n: i32, k: String, m: &CommandMap) -> Result<(), AppError> {
    println!("--- Check Rectangle ---");
    let cmd = CommandType::from_str(&k).unwrap();
    if let Some(vv) = m.get(&k) {
        for v in vv {
            println!("{:?}", v);
            check_generic::validate_params(v, cmd.spec(), &k)?;
        }
    }
    Ok(())
}
