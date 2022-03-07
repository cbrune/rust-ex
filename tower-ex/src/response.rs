//! Sample Response

use crate::prelude::*;

#[derive(Debug)]
/// A sample response object
pub struct SampleResponse {
    /// The value of the response object
    value: usize,
}

impl SampleResponse {
    /// Create new response object
    pub fn new(value: usize) -> Result<SampleResponse, AppError> {
        Ok(SampleResponse { value })
    }

    /// Return the value of a response object
    pub fn value(&self) -> usize {
        self.value
    }
}
