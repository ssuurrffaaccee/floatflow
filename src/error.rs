use std::error::Error;
use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum FFError {
    Op(String),
    Store(String),
    Parse(String),
}
pub type Result<T> = std::result::Result<T, FFError>;

impl Error for FFError {}

impl Display for FFError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FFError::Op(info) => write!(fmt, "op error: {}", info),
            FFError::Store(info) => write!(fmt, "store error: {}", info),
            FFError::Parse(info) => write!(fmt, "parse store: {}", info),
        }
    }
}

impl From<ParseFloatError> for FFError {
    fn from(e: ParseFloatError) -> FFError {
        return FFError::Parse(e.to_string());
    }
}

impl From<ParseIntError> for FFError {
    fn from(e: ParseIntError) -> FFError {
        return FFError::Parse(e.to_string());
    }
}
