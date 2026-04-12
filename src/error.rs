// src/error.rs
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    TypeMismatch { tag: String, col: usize, expected: String, actual: String },
    MissingParameter { tag: String, expected_min: usize, actual: usize },
    InvalidIdentifier { tag: String, val: String },
    InvalidFormat(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "i/o error: {}", err),
            AppError::TypeMismatch { tag, col, expected, actual } => 
                write!(f, "CHECK ERROR: [{}] {} column is type mismatch. except: {}, real: {}", tag, col, expected, actual),
            AppError::MissingParameter { tag, expected_min, actual } => 
                write!(f, "CHECK ERROR: [{}] lack of parameters. except: {}, real: {}", tag, expected_min, actual),
            AppError::InvalidIdentifier { tag, val } =>
                write!(f, "CHECK ERROR: [{}] ID syntax error. Please start '@' string: '{}'", tag, val),
            AppError::InvalidFormat(msg) => 
                write!(f, "CHECK ERROR: {}", msg),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<AppError> for eframe::Error {
    fn from(err: AppError) -> Self {
        eframe::Error::AppCreation(format!("{}", err).into())
    }
}
