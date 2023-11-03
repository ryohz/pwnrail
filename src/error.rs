use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: `{0}`")]
    IoError(#[from] io::Error),
    #[error("`{0}` already exists.")]
    AlreadyExists(String),
    #[error("before initialization of this environment.")]
    BeforeInitEnv,
    #[error("home directory is not found")]
    NoHomeDir,
    #[error("rjql error. `{0}`")]
    RjqlERror(#[from] rjql::error::Error),
    #[error("falied to copy the text to clipboard: `{0}`")]
    CopyToClipboardError(String)
}

#[derive(Error, Debug)]
pub enum VarsModError {}
