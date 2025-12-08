use thiserror::Error;

/// Result type for the crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the converter
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTML parsing error: {0}")]
    HtmlParseError(String),

    #[error("Markdown parsing error: {0}")]
    MarkdownParseError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
