//! Transceiver

use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::stream::Stream;
use tokio::sync::mpsc;

use crate::request::SampleRequest;
use crate::response::SampleResponse;

/// A trait defining the interface for a Tranceiver object
pub trait Transceiver: Stream {}

#[derive(Debug, Clone)]
/// The internal state of a transceiver
pub struct TransceiverState {
    /// transceiver value
    value: usize,

    /// count of items to generate
    n_gen: usize,

    /// delay between generated items in seconds
    delay: u64,

    /// signals shutdown
    done: bool,
}

#[derive(Debug)]
/// A sample receiver object
pub struct SampleTransceiver {
    /// Inner state of a sample tranceiver
    inner: Arc<Mutex<TransceiverState>>,

    /// next request
    request: Arc<Mutex<Option<SampleRequest>>>,

    /// send wakers to the worker
    waker_sender: Option<mpsc::Sender<Waker>>,
}

impl SampleTransceiver {
    /// Create a new sample transreceiver
    pub fn new(value: usize, n_gen: usize, delay: u64) -> SampleTransceiver {
        SampleTransceiver {
            inner: Arc::new(Mutex::new(TransceiverState {
                value,
                n_gen,
                delay,
                done: false,
            })),
            request: Arc::new(Mutex::new(None)),
            waker_sender: None,
        }
    }

    /// emit a response
    pub fn xmit(&mut self, response: SampleResponse) {
        println!("xmit response: {:?}", response);
    }
}

impl Stream for SampleTransceiver {
    type Item = SampleRequest;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // first time through do this:
        // 1. start a thread, passing the context
        // 1.1 return Pending
        // 2. in the thread:
        // 2.1 sleep
        // 2.2 compute a value
        // 2.3 store value in option
        // 2.4 invoke the waker
        //

        {
            let mut request = self.request.lock().expect("taking request lock");
            if let Some(request) = request.take() {
                return Poll::Ready(Some(request));
            }
        }

        {
            let inner = self.inner.lock().expect("taking inner lock");
            if inner.done {
                // we are finished
                return Poll::Ready(None);
            }
        }

        if let Some(sender) = &self.waker_sender {
            if sender.blocking_send(cx.waker().clone()).is_err() {
                println!("Error: receiver dropped");
                return Poll::Ready(None);
            }
        } else {
            let worker_inner = self.inner.clone();
            let worker_request = self.request.clone();
            let (tx, mut rx) = mpsc::channel(2);
            self.waker_sender = Some(tx);
            let (value, n_gen, delay) = {
                let inner = self.inner.lock().expect("taking inner lock");
                (inner.value, inner.n_gen, inner.delay)
            };
            println!("Spawning transceiver task...");
            tokio::spawn(async move {
                println!("transceiver: start");
                let mut count = 0;
                while let Some(waker) = rx.recv().await {
                    if count > n_gen {
                        let mut inner = worker_inner.lock().expect("taking inner lock");
                        inner.done = true;
                        waker.wake();
                        // we are done
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
                    {
                        let mut request = worker_request.lock().expect("taking request lock");
                        *request = Some(SampleRequest::new(value + count));
                    }
                    waker.wake();
                    count += 1;
                }
            });
        }

        Poll::Pending
    }
}

impl Transceiver for SampleTransceiver {}
