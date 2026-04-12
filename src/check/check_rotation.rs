// src/check/check_rotation.rs

use crate::error::AppError;
use crate::CommandMap;

pub fn rotation_core(m: &CommandMap) -> Result<(), AppError> {
    let mut depth = 0;

    if let Some(vv) = m.get("RO") {
        for v in vv {
            let param = match v.get(1) {
                Some(p) => p.as_str(),
                None => return Err(AppError::InvalidFormat("RO: Missing argument. '@ID' or 'E' is required".into())),
            };

            if param == "E" {
                if v.len() > 2 {
                    return Err(AppError::InvalidFormat(format!("RO E: Unexpected argument after 'E': {:?}", &v[2..])));
                }
                depth -= 1;
                if depth < 0 {
                    return Err(AppError::InvalidFormat("Unmatched 'RO E': No rotation block to close".into()));
                }
            } else {
                if !param.starts_with('@') {
                    return Err(AppError::InvalidFormat(format!("RO: ID must start with '@': '{}'", param)));
                }

                let angle_str = match v.get(2) {
                    Some(s) => s,
                    None => return Err(AppError::InvalidFormat(format!("RO: Angle (numeric) is required for start command: {:?}", v))),
                };
                if angle_str.parse::<f32>().is_err() {
                    return Err(AppError::InvalidFormat(format!("RO: Invalid numeric value for angle: '{}'", angle_str)));
                }

                if v.len() > 3 {
                    return Err(AppError::InvalidFormat(format!("RO: Too many arguments for start command: {:?}", &v[3..])));
                }

                depth += 1;
            }
        }
    }

    if depth != 0 {
        return Err(AppError::InvalidFormat(format!("Unclosed RO: {} nested rotation block(s) remaining", depth)));
    }

    Ok(())
}
