// - outer most box is the "Server" and takes three arguments:
// -- a "Service" -- this is the message dispatcher
// -- a "Transceiver" -- sends/dreceives messages
// -- a "Signal" -- thing that can signal a the server to shutdown

// Server Trait
// Service Trait
// Transceiver Trait
// Signal

// signal: Option<F>,
// where F: Future<Output = ()>,

use tokio::runtime::Builder;

use sample::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("sample-server")
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    runtime.block_on(async {
        println!("Hello, world!");

        let err = AppError::SampleError("for grins".to_owned());
        println!("Err: {:?}", err);

        // let svc = SampleService::new();
    });

    Ok(())
}
