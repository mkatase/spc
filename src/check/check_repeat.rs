// src/check/check_repeat.rs

use crate::error::AppError;
use crate::check::check_generic;
use crate::check::check_lib::CommandType;
use crate::CommandMap;

pub fn repeat_core(k: String, m: &CommandMap) -> Result<(), AppError> {
    println!("--- Check Repeat ---");
    let cmd = CommandType::from_str(&k).unwrap();
    if let Some(vv) = m.get(&k) {
        for v in vv {
            println!("{:?}", v);
            let id = v.get(1).map(|s| s.as_str()).unwrap_or("");
            if !id.starts_with('@') {
                return Err(AppError::InvalidFormat(format!("{} ID error: '{}'", k, id)));
            }
            check_generic::validate_params(v, cmd.spec(), &k)?;

            if v.len() > cmd.spec().len() {
                return Err(AppError::InvalidFormat(format!("{} length error", k)));
            }
        }
    }
            
    Ok(())
}
