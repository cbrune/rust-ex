use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::EnvFilter;

use ispgw::AppError;
use ispgw::IspgwdConfig;
use ispgw::Service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;

    let config = gather_config()?;

    info!("Starting service!");

    run_app(config).context("Problems running app")?;

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

fn gather_config() -> Result<IspgwdConfig, anyhow::Error> {
    let arguments = CliArgs::from_args();
    let config_file = fs::File::open(&arguments.config_file).context(format!(
        "Error opening config file {}",
        &arguments.config_file.display()
    ))?;

    let mut config: IspgwdConfig = serde_yaml::from_reader(config_file).context(format!(
        "Error parsing config file: {}",
        &arguments.config_file.display()
    ))?;

    if arguments.simulation {
        config.simulation = arguments.simulation;
    }

    info!("Config: {:?}", config);

    Ok(config)
}

fn run_app(config: IspgwdConfig) -> Result<(), AppError> {
    info!("Starting daemon with config: {:?}", config);

    let mut service = Service::new(config)?;

    service.run()
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(author)]
/// ISP gateway daemon
///
struct CliArgs {
    /// Emit debug logging
    #[structopt(short, long)]
    debug: bool,

    /// Confiugration file
    #[structopt(short, long)]
    config_file: PathBuf,

    /// enable simluation backend
    #[structopt(short, long)]
    simulation: bool,
}
