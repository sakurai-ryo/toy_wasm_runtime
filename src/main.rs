use anyhow::Result;
mod cmd;
mod errors;
pub mod exec;

fn main() -> Result<()> {
    cmd::exec()
}
