use crate::utils::ParseQueue;

use super::error::{PError, PResult};

pub fn digit(input: &mut ParseQueue) -> PResult<usize> {
    match input.len() {
        0 => Err(PError::EndOfInput),
        _ => input
            .consume_while(|c| c.is_numeric())
            .parse::<usize>()
            .map_err(|_| match input.peek() {
                Some(c) => PError::InvalidChar(c),
                None => PError::EndOfInput,
            }),
    }
}

pub fn alpha1(input: &mut ParseQueue) -> PResult<String> {
    let out = alpha0(input)?;
    match out.len() {
        0 => Err(PError::InvalidChar(input.peek().unwrap())),
        _ => Ok(out),
    }
}

pub fn alpha0(input: &mut ParseQueue) -> PResult<String> {
    match input.len() {
        0 => Err(PError::EndOfInput),
        _ => Ok(input.consume_while(|c| c.is_alphabetic())),
    }
}

pub fn whitespace1(input: &mut ParseQueue) -> PResult<String> {
    match input.len() {
        0 => Err(PError::EndOfInput),
        _ => {
            let str = input.consume_while(|c| c.is_whitespace());
            match str.len() {
                0 => Err(PError::InvalidChar(input.peek().unwrap())),
                _ => Ok(str),
            }
        }
    }
}

pub fn whitespace0(input: &mut ParseQueue) -> PResult<String> {
    match input.len() {
        0 => Err(PError::EndOfInput),
        _ => Ok(input.consume_while(|c| c.is_whitespace())),
    }
}
