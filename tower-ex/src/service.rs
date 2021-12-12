//! Server Service

use std::future;
use std::pin::Pin;
use std::task::Poll;

use tower::Service;

use crate::prelude::*;

#[derive(Default, Debug)]
pub struct SampleService {
    count: usize,
}

impl SampleService {
    pub fn new() -> SampleService {
        SampleService::default()
    }
}

impl Service<SampleRequest> for SampleService {
    type Response = SampleResponse;
    type Error = AppError;
    type Future = Pin<Box<dyn future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: SampleRequest) -> Self::Future {
        // create the response inside a future
        let future = async move {
            let response = SampleResponse::new(req.val() * 3);
            response
        };

        // increment our processing count
        self.count += 1;

        // return the future in a box
        Box::pin(future)
    }
}
