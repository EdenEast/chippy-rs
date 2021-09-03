use std::num::ParseIntError;

use thiserror::Error;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum LineError {
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Invalid Address: {0}")]
    InvalidAddress(#[from] ParseIntError),

    #[error("Wrong jump register")]
    WrongJumpRegister,

    #[error("Invalid Register: {0}")]
    InvalidRegister(String),

    #[error("Wrong number of arguments: expected {0}, got {1}")]
    WrongNumberOfArguments(usize, usize),

    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("LineError at {0}: {1}")]
    Line(usize, LineError),
}
