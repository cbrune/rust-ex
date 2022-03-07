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

async fn run() -> Result<(), AppError> {
    println!("Hello, world!");

    let svc = SampleService::new();

    let transceiver1 = SampleTransceiver::new(1, 3, 2);
    let transceiver2 = SampleTransceiver::new(100, 3, 2);

    // A shutdown signal
    let (tx, rx) = tokio::sync::oneshot::channel();

    let server = Server::builder()
        .with_transceiver(transceiver1)
        .with_transceiver(transceiver2)
        .serve_with_shutdown(svc, async move {
            let _ = rx.await;
        })?;

    // create a task to trigger shutdown in the future
    tokio::spawn(async move {
        for i in 0..7 {
            println!("shutdown task: {}: sleeping ...", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        println!("shutdown task: shutting down ...");
        let _ = tx.send(());
    });

    println!("main: waiting for server to shutdown ...");
    match server.await {
        Ok(_) => {
            println!("Server exits success");
            Ok(())
        }
        Err(e) => {
            println!("Server exits error");
            Err(AppError::ServerRuntimeFailure(e.to_string()))
        }
    }

    /*
        for i in 0..3 {
        let request = SampleRequest::new(i);
        println!("request: {:?}", request);
        let response = svc.ready().await.unwrap().call(request).await.unwrap();

        println!("response: {:?}", response);
        println!("service: {:?}", svc);
    }
         */
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("sample-server")
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    runtime.block_on(run())?;

    Ok(())
}
