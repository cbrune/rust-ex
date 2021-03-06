//! Server Service

use std::future;
use std::pin::Pin;
use std::task::Poll;

use tower::Service;

use crate::prelude::*;

#[derive(Default, Debug)]
/// A sample service
///
/// This simple service takes a SampleRequest, multiplies its value by
/// 3, and returns a SampleResult.
pub struct SampleService {
    count: usize,
}

impl SampleService {
    /// Create a new sample service
    pub fn new() -> SampleService {
        SampleService::default()
    }
}

type SampleRequestFuture = dyn future::Future<Output = Result<SampleResponse, AppError>>;

impl Service<SampleRequest> for SampleService {
    type Response = SampleResponse;
    type Error = AppError;
    type Future = Pin<Box<SampleRequestFuture>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: SampleRequest) -> Self::Future {
        // create the response inside a future
        let future = async move { SampleResponse::new(req.value() * 3) };

        // increment our processing count
        self.count += 1;

        // return the future in a box
        Box::pin(future)
    }
}
