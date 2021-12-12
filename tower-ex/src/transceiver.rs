//! Transceiver

use std::sync::{Arc, Mutex};

/// A trait defining the interface for a Tranceiver object
pub trait Transceiver {}

#[derive(Debug, Clone)]
/// The internal state of a transceiver
pub struct TransceiverState {
    /// transceiver value
    value: usize,

    /// count of items generated
    count: usize,

    /// maximum delay between generated items
    max_delay: usize,
}

#[derive(Debug, Clone)]
/// A sample receiver object
pub struct SampleTransceiver {
    /// Inner state of a sample tranceiver
    inner: Arc<Mutex<TransceiverState>>,
}

impl SampleTransceiver {
    /// Create a new sample receiver
    pub fn new(value: usize, count: usize, max_delay: usize) -> SampleTransceiver {
        SampleTransceiver {
            inner: Arc::new(Mutex::new(TransceiverState {
                value,
                count,
                max_delay,
            })),
        }
    }
}

impl Transceiver for SampleTransceiver {}
