use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppInitError {
    #[error("Your home directory is not found")]
    HomeNotFound,
    #[error("Io Error: `{0}`")]
    IoError(String),
    #[error("initialization is already done")]
    InitAlreadyDone,
    #[error("failed to init dynamic config: `{0}`")]
    DynConfInitError(String),
}

#[derive(Error, Debug)]
pub enum DynConfInitError {
    #[error("Io Error: `{0}`")]
    IoError(String),
    #[error("failed to parse config to toml: `{0}`")]
    TomlError(String),
}
