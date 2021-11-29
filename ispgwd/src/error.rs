//! Sample Error codes

use std::io;
use std::net;

use thiserror::Error;

/// Enum of error types used throughout the server
#[derive(Error, Debug)]
pub enum AppError {
    /// Sample error
    #[error("Sample error: {0}")]
    SampleError(String),

    /// Interface name not found
    #[error("Interface name not found: {0}")]
    InterfaceNotFound(String),

    /// Std IO error
    #[error("I/O error")]
    IoError(#[from] io::Error),

    /// Network address parsing error
    #[error("Unable to parse network address")]
    AddrParseError(#[from] net::AddrParseError),
}
