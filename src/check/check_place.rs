// src/check/check_place.rs

use crate::error::AppError;
use crate::check::check_generic;
use crate::check::check_lib::CommandType;
use crate::CommandMap;

pub fn place_core(_n: i32, k: String, m: &CommandMap) -> Result<(), AppError> {
    println!("--- Check Placement ---");
    let cmd = CommandType::from_str(&k).unwrap();
    if let Some(vv) = m.get(&k) {
        for v in vv {
            println!("{:?}", v);
            let id = v.get(1).map(|s| s.as_str()).unwrap_or("");
            if !id.starts_with('@') {
                return Err(AppError::InvalidFormat(format!("{} ID error: '{}'", k, id)));
            }
            if v.len() > cmd.spec().len() {
                return Err(AppError::InvalidFormat(format!("[PL] Too many arguments （Expected: 4, Actual: {}）: {:?}", v.len(), v)));
            }
            check_generic::validate_params(v, cmd.spec(), &k)?;
        }
    }
    Ok(())
}
