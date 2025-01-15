use std::{fmt, result};

#[derive(Debug)]
pub enum PError {
    InvalidChar(char),
    EndOfInput,
}

impl std::error::Error for PError {}

impl fmt::Display for PError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PError::InvalidChar(c) => write!(f, "Invalid char '{}'", c),
            PError::EndOfInput => write!(f, "End of input"),
        }
    }
}

pub type PResult<T> = result::Result<T, PError>;
