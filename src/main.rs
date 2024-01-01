use anyhow::Result;
mod cmd;
mod errors;
pub mod exec;
use clap::Parser;
use std::path::PathBuf;

/// Run a wasm file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the wasm file to run.
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    cmd::exec(PathBuf::from(args.file))?;
    Ok(())
}
