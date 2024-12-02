use thiserror::Error;

#[derive(Error, Debug)]
pub enum KodikError {
    #[error("Token error: {0}")]
    TokenError(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("No results found: {0}")]
    NoResults(String),

    #[error("Invalid ID type: {0}")]
    InvalidIdType(String),

    #[error("Parser error: {0}")]
    ParserError(String),
} 