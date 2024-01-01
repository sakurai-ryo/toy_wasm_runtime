use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Unknown error")]
    Unknown,
}
