use std::{fmt, result};

#[derive(Debug)]
pub enum XPathError {
    ParseError { exp: String },
    Error { msg: String },
}

impl std::error::Error for XPathError {}

impl fmt::Display for XPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XPathError::ParseError { exp } => write!(f, "Failed to parse expression: '{}'", exp,),
            XPathError::Error { msg } => write!(f, "Error: {}", msg),
        }
    }
}

pub type XPathResult<T> = result::Result<T, XPathError>;
