//! tower-rs experiment application

#![warn(missing_docs)]

mod error;
mod request;
mod response;
mod server;
mod service;
mod transceiver;

pub mod prelude {
    //! Common things to include in all modules

    pub use crate::error::AppError;
    pub use crate::request::SampleRequest;
    pub use crate::response::SampleResponse;
    pub use crate::server::{Builder, Server};
    pub use crate::service::SampleService;
    pub use crate::transceiver::{SampleTransceiver, Transceiver};
}
