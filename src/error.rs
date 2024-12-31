// error.rs
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(mongodb::error::Error),
    Request(reqwest::Error),
    Parse(String),
    Other(String),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Request(e) => write!(f, "Request error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Request(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Parse(err.to_string())
    }
}
