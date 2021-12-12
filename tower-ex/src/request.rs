//! Sample Request

#[derive(Debug)]
/// A Sample Request Object
pub struct SampleRequest {
    /// The value within the request
    value: usize,
}

impl SampleRequest {
    /// Create a new sample request
    pub fn new(value: usize) -> SampleRequest {
        SampleRequest { value }
    }

    /// Return the value of the request
    pub fn value(&self) -> usize {
        self.value
    }
}
