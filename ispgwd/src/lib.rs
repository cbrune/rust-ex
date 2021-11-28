//! Sample application

#![warn(missing_docs)]

mod config;
mod error;
mod net;
mod service;

pub use config::*;
pub use error::AppError;
pub use service::Service;

pub mod prelude {
    //! Common things to include in all modules

    pub use pnet::datalink::NetworkInterface;
    pub use tracing::{debug, error, info, warn};

    pub use crate::config::*;
    pub use crate::error::AppError;
}
