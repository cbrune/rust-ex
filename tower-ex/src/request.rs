//! Sample Request

#[derive(Debug)]
pub struct SampleRequest(usize);

impl SampleRequest {
    pub fn new(val: usize) -> SampleRequest {
        SampleRequest(val)
    }

    pub fn val(&self) -> usize {
        self.0
    }
}
