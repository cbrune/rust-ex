//! Sample Response

use crate::prelude::*;

#[derive(Debug)]
pub struct SampleResponse(usize);

impl SampleResponse {
    pub fn new(val: usize) -> Result<SampleResponse, AppError> {
        Ok(SampleResponse(val))
    }

    pub fn val(&self) -> usize {
        self.0
    }
}
