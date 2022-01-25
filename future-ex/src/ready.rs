//! A future indicating readiness of an operation

use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;

use crate::prelude::*;

/// A future indicating readiness
#[derive(Debug, Clone)]
pub struct Ready {
    poll_ready: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
    result: Result<(), AppError>,
}

impl Ready {
    pub fn new(fail: bool) -> Ready {
        info!("Creating a new future");
        Ready {
            poll_ready: Arc::new(AtomicBool::new(false)),
            waker: Arc::new(Mutex::new(None)),
            result: match fail {
                true => Err(AppError::PollingError("Failed future polling".to_string())),
                false => Ok(()),
            },
        }
    }
}

impl Future for Ready {
    type Output = Result<(), AppError>;

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        info!("Polling");

        let mut waker = self.waker.lock().unwrap();
        if self.poll_ready.load(Ordering::Relaxed) {
            info!("poll: ready!");
            Poll::Ready(self.result.clone())
        } else if waker.is_none() {
            info!("poll: setting initial waker");
            *waker = Some(context.waker().clone());
            // start the polling thread ...
            let thread_waker = context.waker().clone();
            let thread_waker2 = context.waker().clone();
            let thread_ready = self.poll_ready.clone();
            thread::spawn(move || {
                info!("Ready thread starting");
                let count = AtomicUsize::new(0);

                // bogus wake to trigger a poll event
                thread_waker.wake();
                while count.fetch_add(1, Ordering::Relaxed) < 100000000 {}
                info!("Ready thread complete, waking");
                thread_ready.store(true, Ordering::Relaxed);
                thread_waker2.wake();
            });
            Poll::Pending
        } else {
            info!("poll: not ready");
            Poll::Pending
        }
    }
}
