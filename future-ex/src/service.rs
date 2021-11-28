//! Future service experiment

use crate::prelude::*;

use crate::ready::Ready;

/// Service struct
pub struct Service;

impl Service {
    /// Create a new service
    pub fn new() -> Service {
        info!("Creating a new service");
        Service {}
    }

    /// Start and run the service
    pub async fn run(&mut self) -> Result<(), AppError> {
        info!("Running a service");

        let ready = Ready::new(false);
        ready.await?;

        let ready_false = Ready::new(true);
        ready_false.await
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}
