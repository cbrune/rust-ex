//! Application Server

// server is a  future

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::prelude::*;

pub struct Server<S, T> {
    service: S,
    transceivers: Vec<T>,
    signal: Pin<Box<Receiver<()>>>,
}

#[derive(Debug)]
pub struct Builder<T> {
    transceivers: Vec<T>,
}

impl<T> Server<(), T> {
    pub fn builder() -> Builder<T> {
        Builder {
            transceivers: Vec::new(),
        }
    }
}

impl<S, T> Unpin for Server<S, T> {}

impl<S, T> Future for Server<S, T> {
    type Output = Result<(), AppError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Starting server poll loop....");
        loop {
            // any transceivers have work?
            // received shutdown signal?
            let signal = Pin::as_mut(&mut self.signal);
            match signal.poll(cx) {
                Poll::Ready(_result) => {
                    println!("Received shutdown signal");
                    return Poll::Ready(Ok(()));
                }
                Poll::Pending => {
                    println!("poll: Returning Pending");
                    return Poll::Pending;
                }
            }
        }
    }
}

impl<T> Builder<T> {
    pub fn with_transceiver(mut self, transceiver: T) -> Builder<T> {
        self.transceivers.push(transceiver);
        self
    }

    pub fn serve_with_shutdown<S>(self, service: S, signal: Receiver<()>) -> Server<S, T> {
        Server {
            service,
            transceivers: self.transceivers,
            signal: Box::pin(signal),
        }
    }
}
