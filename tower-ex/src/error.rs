//! Sample Error codes

use thiserror::Error;

/// Enum of error types used throughout the server
#[derive(Error, Debug)]
pub enum AppError {
    /// Sample error
    #[error("Sample error: {0}")]
    SampleError(String),
}
