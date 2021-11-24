//! Future experiments Error codes

use thiserror::Error;

/// Enum of error types used throughout the application
#[derive(Error, Debug, Clone)]
pub enum AppError {
    /// Polling error
    #[error("Polling error: {0}")]
    PollingError(String),
}
