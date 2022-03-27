// use sudoku::prelude::*;

use std::path::PathBuf;

use clap::Parser;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

use sudoku::prelude::*;

/// Sudoku Puzzle Solver
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of input puzzle file
    #[clap(short, long)]
    puzzle_file: PathBuf,

    /// Debug output
    #[clap(short, long)]
    debug: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    setup(&args)?;

    let mut puzzle = Puzzle::new(&args.puzzle_file)?;

    info!("Using puzzle:\n{}", puzzle);
    debug!("Using puzzle deubg:\n{:?}", puzzle);

    match puzzle.solve() {
        Ok(iters) => {
            info!("Solved puzzle iterations: {}\n{}", iters, puzzle);
            Ok(())
        }
        Err((e, iter)) => {
            error!("Failed to solve puzzle: {:?}\n{}", e, puzzle);
            error!("Error puzzle state:\n{:?}", puzzle);
            error!("Total iterations: {}", iter);
            Err(e.into())
        }
    }
}

fn setup(args: &Args) -> Result<(), anyhow::Error> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    if args.debug {
        std::env::set_var("RUST_LOG", "debug")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    Ok(())
}
