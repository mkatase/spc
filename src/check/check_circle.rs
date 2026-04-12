// src/check/check_line.rs

use crate::error::AppError;
use crate::check::check_generic;
use crate::check::check_lib::CommandType;
use crate::CommandMap;

pub fn circle_core(_n: i32, k: String, m: &CommandMap) -> Result<(), AppError> {
    println!("--- Check Circle ---");
    let cmd = CommandType::from_str(&k).unwrap();
    if let Some(vv) = m.get(&k) {
        for v in vv {
            println!("{:?}", v);
            check_generic::validate_params(v, cmd.spec(), &k)?;
        }
    }

    Ok(())
}
