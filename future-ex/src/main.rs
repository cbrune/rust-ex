use anyhow::Context;
use tracing_subscriber::EnvFilter;

use future_ex::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;

    info!("Starting application!");

    run_app().await.context("Problems running app")?;

    Ok(())
}

fn setup() -> Result<(), anyhow::Error> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

async fn run_app() -> Result<(), AppError> {
    info!("Running application");

    let mut service = Service::new();

    service.run().await
}
