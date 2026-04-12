// src/check/check_core.rs
use crate::error::AppError;
use crate::CommandMap;
use crate::check::check_lib::ParamType;
use crate::check::check_lib::{
    Checker, CommandType, LATEST_VERSION
};

pub fn check_ops(n: i32, m: &CommandMap) {
    println!(" version : {}", n);
    println!("--- check ops ---");
    let checker = Checker::new(n);
    for k in m.keys() {
        //println!("{}", k);
        if let Some(cmd) = CommandType::from_str(k) {
            match cmd {
                _ => {
                    if !checker.is_allowed(cmd) {
                        println!(" Skipped: {} is not allowed", k);
                    }
                }
            }
        }
    }
}

pub fn get_ver(k: String, m: &CommandMap) -> Result<i32, AppError> {
    println!("--- get version ---");
    let v = match m.get(&k) {
        Some(n) => n,
        None => return Ok(1),
    };

    match v.len() {
        0 => Ok(1),
        1 => {
            if v[0].len() == 2 {
                let y = v[0][1].clone().parse::<i32>().unwrap();
                if y <= LATEST_VERSION {
                    Ok(y)
                } else {
                    Err(AppError::InvalidFormat(
                            "Invalid VN version value".to_string()))
                }
            } else {
                Err(AppError::InvalidFormat(
                        "VN value is missing".to_string()))
            }
        },
        _ => Err(AppError::InvalidFormat(
                "Duplicate VN declaration".to_string()))
    }
}

pub fn validate_numeric(v: &[String], range: std::ops::RangeInclusive<usize>, tag: &str)
    -> Result<(), AppError> {
    for i in range {
        if let Some(val) = v.get(i) {
            val.parse::<f32>().map_err(|_| {
                AppError::InvalidFormat(format!("{} parameter is not a number at index {}: {}", tag, i, val))
            })?;
        } else {
            return Err(AppError::InvalidFormat(format!("{} column count mismatch at index {}", tag, i)));
        }
    }
    Ok(())
}

pub fn check_options(options: &[String]) {
    if options.is_empty() {
        println!("  No options");
    } else {
        println!("  Options: {:?}", options);
    }
}

pub fn validate_params(v: &[String], spec: &[ParamType], tag: &str) -> Result<(), AppError> {

    for (i, val) in v.iter().enumerate() {
        if let Some(expected) = spec.get(i) {
            let idx = i + 1;
            match expected {
                ParamType::GroupId => {
                    if !val.starts_with('@') {
                        return Err(AppError::InvalidFormat(
                            format!("[{}] Column {}: ID must start with '@': {}", tag, idx, val)
                        ));
                    }
                },
                ParamType::Keyword => {
                    if val != "E" {
                        return Err(AppError::InvalidFormat(format!("[{}] Reserved keyword 'E' is required: {}", tag, val)));
                    }
                },
                ParamType::Int => {
                    val.parse::<i32>().map_err(|_| {
                        AppError::InvalidFormat(format!("[{}] Column {}: Numeric value required: {}", tag, idx, val))
                    })?;
                },
                ParamType::Float => {
                    val.parse::<f32>().map_err(|_| {
                        AppError::InvalidFormat(format!("[{}] Column {}: Numeric value required: {}", tag, idx, val))
                    })?;
                },
                ParamType::Color => {
                    if val.parse::<f32>().is_ok() {
                        return Err(AppError::InvalidFormat(format!("Unexpected numeric value in color field: {}", val)));
                    }
                },
                ParamType::Any => {},
                ParamType::Str => {}
            }
        }
    }
    Ok(())
}

pub fn settings_core(k: String, m: &CommandMap) -> Result<(), AppError> {
    if let Some(vv) = m.get(&k) {
        let cmd = match CommandType::from_str(&k) {
            Some(c) => c,
            None => return Ok(()),
        };
        let spec = cmd.spec();
        for v in vv {
            validate_params(v, spec, &k)?;

            if v.len() > spec.len() {
                check_options(&v[spec.len()..]);
            }
        }
    }
    Ok(())
}
