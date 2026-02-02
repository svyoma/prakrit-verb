use thiserror::Error;

/// Application-level errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Conjugation error: {0}")]
    Conjugation(#[from] ConjugationError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Conjugation-specific errors
#[derive(Error, Debug)]
pub enum ConjugationError {
    #[error("Empty verb root provided")]
    EmptyRoot,

    #[error("Invalid character in verb root: {0}")]
    InvalidCharacter(char),

    #[error("Verb root is too short")]
    TooShort,
}

pub type Result<T> = std::result::Result<T, AppError>;
pub type ConjugationResult<T> = std::result::Result<T, ConjugationError>;
