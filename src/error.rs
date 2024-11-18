use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackerError {
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<sqlx::Error> for TrackerError {
    fn from(err: sqlx::Error) -> Self {
        TrackerError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for TrackerError {
    fn from(err: serde_json::Error) -> Self {
        TrackerError::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for TrackerError {
    fn from(err: std::io::Error) -> Self {
        TrackerError::Storage(err.to_string())
    }
}

impl From<aws_sdk_s3::Error> for TrackerError {
    fn from(err: aws_sdk_s3::Error) -> Self {
        TrackerError::Storage(err.to_string())
    }
}
