use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Not found")]
    NotFound,
}
