use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppConfigError {
    #[error("failed to init app: `{0}`")]
    AppInitError(String),
    #[error("failed to read dynamic config: `{0}`")]
    ReadDynConfError(String),
}

#[derive(Error, Debug)]
pub enum ReadDynConfError {
    #[error("failed to open dynamic config file: `{0}`")]
    OpenError(String),
    #[error("failed to parse toml file as dynamic config: `{0}`")]
    ParseError(String),
}

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
    #[error("failed to init shell history: `{0}`")]
    ShellHistInitError(String),
}

#[derive(Error, Debug)]
pub enum DynConfInitError {
    #[error("Io Error: `{0}`")]
    IoError(String),
    #[error("failed to parse config to toml: `{0}`")]
    TomlError(String),
}

#[derive(Error, Debug)]
pub enum ShellHistInitError {
    #[error("Io Error: `{0}`")]
    IoError(String),
}
