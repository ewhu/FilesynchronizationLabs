// src/main.rs
/*
 * Main executable for FilesynchronizationLabs
 */

use clap::Parser;
use filesynchronizationlabs::{Result, run};

#[derive(Parser)]
#[command(version, about = FilesynchronizationLabs - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
