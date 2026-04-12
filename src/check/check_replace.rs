// src/check/check_replace.rs

use crate::error::AppError;
use crate::check::check_generic;
use crate::check::check_lib::CommandType;
use crate::CommandMap;

pub fn replace_core(k: String, m: &CommandMap) -> Result<(), AppError> {
    println!("--- Check Replace ---");
    let cmd = CommandType::from_str(&k).unwrap();
    if let Some(vv) = m.get(&k) {
        for v in vv {
            if v.len() != cmd.spec().len() {
                return Err(AppError::InvalidFormat(format!("{}: Invalid column count（Expected: 3, Actual: {}）", k, v.len())));
            }
            check_generic::validate_params(v, cmd.spec(), &k)?;
        }
    }
    Ok(())
}
