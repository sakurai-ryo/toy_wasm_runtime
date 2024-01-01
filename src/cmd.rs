use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::ErrorKind;

use crate::errors::ExecError;
use crate::exec::buffer::Buffer;
use crate::exec::module::ModuleNode;

/// Run a wasm file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the wasm file to run.
    #[arg(short, long)]
    file: String,
}

pub fn exec() -> Result<()> {
    let args = Args::parse();

    let file = fs::read(&args.file);
    let file_content = match file {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err(ExecError::FileNotFound(args.file).into());
            }
            eprintln!("Unknown error: {}", e);
            return Err(ExecError::Unknown.into());
        }
    };

    let mut buffer = Buffer::new(file_content);
    let mut module = ModuleNode::new();
    module.load(&mut buffer)?;

    println!("{:?}", module);

    return Ok(());
}
