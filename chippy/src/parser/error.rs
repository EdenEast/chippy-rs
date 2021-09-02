use thiserror::Error;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}
