use std::{fmt, result};

#[derive(Debug)]
pub enum DomError {
    ParseError { exp: String },
    Error { msg: String },
}

impl std::error::Error for DomError {}

impl fmt::Display for DomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomError::ParseError { exp } => write!(f, "Failed to parse expression: '{}'", exp,),
            DomError::Error { msg } => write!(f, "Error: {}", msg),
        }
    }
}

pub type DomResult<T> = result::Result<T, DomError>;
