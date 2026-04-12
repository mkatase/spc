// src/check/check_group.rs

use crate::error::AppError;
use crate::CommandMap;

pub fn group_core(m: &CommandMap) -> Result<(), AppError> {
    let mut depth = 0;

    if let Some(vv) = m.get("GR") {
        for v in vv {
            let param = match v.get(1) {
                Some(p) => p.as_str(),
                None => return Err(AppError::InvalidFormat("GR E: Missing argument. '@ID' or 'E' is required".into())),
            };

            if param == "E" {
                if v.len() > 2 {
                    return Err(AppError::InvalidFormat(format!("GR E: Unexpected parameters after 'E': {:?}", &v[2..])));
                }
                depth -= 1;
                if depth < 0 {
                    return Err(AppError::InvalidFormat("Unmatched 'GR E': No group to close".into()));
                }
            } else {
                if !param.starts_with('@') {
                    return Err(AppError::InvalidFormat(format!("GR: ID must start with '@': '{}'", param)));
                }
                if v.len() > 2 {
                    return Err(AppError::InvalidFormat(format!("GR: Too many arguments for group start: {:?}", &v[2..])));
                }
                depth += 1;
            }
        }
    }

    if depth != 0 {
        return Err(AppError::InvalidFormat(format!("Unclosed GR: {} nested group(s) remaining", depth)));
    }

    Ok(())
}
