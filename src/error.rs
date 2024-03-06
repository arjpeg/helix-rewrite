use thiserror::Error;

/// A wrapper over anyhow's `Result` type
pub type Result<T> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Encountered an unknown symbol, '{}'", .0)]
    InvalidSymbol(String),
}
