//! Transceiver

use std::sync::{Arc, Mutex};

pub trait Transceiver {}

#[derive(Debug, Clone)]
pub struct TransceiverState {
    value: usize,
    count: usize,
    max_delay: usize,
}

#[derive(Debug, Clone)]
pub struct SampleTransceiver {
    inner: Arc<Mutex<TransceiverState>>,
}

impl SampleTransceiver {
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
