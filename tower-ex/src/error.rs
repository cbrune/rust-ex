//! Sample Error codes

use thiserror::Error;

/// Enum of error types used throughout the server
#[derive(Error, Debug)]
pub enum AppError {
    /// Server config error
    #[error("Server config error: {0}")]
    ServerConfig(String),

    /// Server runtime failure
    #[error("Server runtime failure: {0}")]
    ServerRuntimeFailure(String),
}
