//! Future experiment application

#![warn(missing_docs)]

mod error;
mod ready;
mod service;

pub mod prelude {
    //! Common things to include in all modules

    pub use tracing::{debug, error, info, warn};

    pub use crate::error::AppError;
    pub use crate::service::Service;
}
