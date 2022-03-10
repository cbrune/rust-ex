//! Application Server

// server is a  future

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::prelude::*;

/// A sample server object
pub struct Server<S, T> {
    /// Service handlers for the server
    _service: S,

    /// Transeivers for the server
    _transceivers: Vec<T>,

    /// Next transceiver
    _next_transceriver: usize,

    /// Future used to shutdown the server
    signal: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

#[derive(Debug)]
/// A builder for constructing servers
pub struct Builder<T> {
    transceivers: Vec<T>,
}

impl<T> Server<(), T> {
    /// Creates a new builder
    pub fn builder() -> Builder<T> {
        Builder {
            transceivers: Vec::new(),
        }
    }
}

// Needed to take the Pin::as_mut() of self.signal
// see: https://users.rust-lang.org/t/take-in-impl-future-cannot-borrow-data-in-a-dereference-of-pin/52042
impl<S, T> Unpin for Server<S, T> {}

impl<S, T> Future for Server<S, T> {
    type Output = Result<(), AppError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Starting server poll loop....");

        // any transceivers have work?
        // received shutdown signal?
        let signal = Pin::as_mut(&mut self.signal);
        match signal.poll(cx) {
            Poll::Ready(_result) => {
                println!("Received shutdown signal");
                Poll::Ready(Ok(()))
            }
            Poll::Pending => {
                println!("shutdown: Returning Pending");
                // check transceivers

                Poll::Pending
            }
        }
    }
}

impl<T> Builder<T> {
    /// Add a transceiver to the server
    pub fn with_transceiver(mut self, transceiver: T) -> Builder<T> {
        self.transceivers.push(transceiver);
        self
    }

    /// Start the server and include a shutdown signal
    pub fn serve_with_shutdown<S, F>(self, service: S, signal: F) -> Result<Server<S, T>, AppError>
    where
        F: Future<Output = ()> + 'static,
    {
        if self.transceivers.is_empty() {
            return Err(AppError::ServerConfig(
                "Trying to start server with no transceivers".to_owned(),
            ));
        }

        Ok(Server {
            _service: service,
            _transceivers: self.transceivers,
            _next_transceriver: 0,
            signal: Box::pin(signal),
        })
    }
}
