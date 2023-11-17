use std::io;

use thiserror::Error;

use crate::output::error_prefix;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error around config")]
    AppConfigError(AppConfigError),
    #[error("failed to init current directory as a workpsace")]
    InitCurrentDirAsWorkspaceError(InitCurrentDirAsWorkspaceError),
    #[error("failed to use current directory as a workspace")]
    UseCurrentDirAsWorkspaceError(UseCurrentDirAsWorkspaceError),
}

#[derive(Error, Debug)]
pub enum AppConfigError {
    #[error("failed to init app")]
    AppInitError(AppInitError),
    #[error("failed to read dynamic config")]
    ReadDynConfError(ReadDynConfError),
}

#[derive(Error, Debug)]
pub enum ReadDynConfError {
    #[error("failed to open dynamic config file")]
    OpenError(io::Error),
    #[error("failed to read dynamic config content from its file data")]
    ReadError(io::Error),
    #[error("failed to parse toml file as dynamic config")]
    ParseError(toml::de::Error),
}

#[derive(Error, Debug)]
pub enum AppInitError {
    #[error("failed to get if app config directory")]
    CheckAppConfPresenceError(io::Error),
    #[error("initialization is already done")]
    InitAlreadyDone,
    #[error("failed to create app config directory")]
    AppConfDirCreateError(io::Error),
    #[error("failed to init dynamic config: `{0}`")]
    DynConfInitError(DynConfInitError),
    #[error("failed to init shell history: `{0}`")]
    ShellHistInitError(ShellHistInitError),
}

#[derive(Error, Debug)]
pub enum DynConfInitError {
    #[error("failed to create dynamic config file")]
    CreateError(io::Error),
    #[error("failed to parse DynamicConfig as Toml")]
    ParseError(toml::ser::Error),
    #[error("failed to write dynamic config to config file")]
    WriteError(io::Error),
}

#[derive(Error, Debug)]
pub enum ShellHistInitError {
    #[error("failed to create shell history file")]
    CreateError(io::Error),
}

#[derive(Error, Debug)]
pub enum UpdateDynConfFileError {
    #[error("failed to parse toml file as dynamic config")]
    ParseError(toml::ser::Error),
    #[error("failed to open dynamic config file")]
    OpenError(io::Error),
    #[error("failed to write new dynamic config to the file")]
    WriteError(io::Error),
}

#[derive(Error, Debug)]
pub enum UseCurrentDirAsWorkspaceError {
    #[error("failed to get current directory")]
    GetCurrentDirError(io::Error),
    #[error("failed to get whether a workspace manager exists in current directory")]
    CheckMgrPresenceError(io::Error),
    #[error("failed to update dynamic config")]
    UpdateDynConfFileError(UpdateDynConfFileError),
    #[error("the workspace manager doesn't exist in the current directory")]
    BeforeInitError,
}

#[derive(Error, Debug)]
pub enum InitCurrentDirAsWorkspaceError {
    #[error("failed to get current directory")]
    GetCurrentDirError(io::Error),
    #[error("failed to create new workspace")]
    CreateNewWorkspaceError(CreateNewWorkspaceError),
    #[error("failed to use current directory as a workspace")]
    UseCurrentDirAsWorkspaceError(UseCurrentDirAsWorkspaceError),
}

#[derive(Error, Debug)]
pub enum CreateNewWorkspaceError {
    #[error("failed to get whether a workspace manager already exists in current directory")]
    CheckMgrPresenceError(io::Error),
    #[error("a workspace manager directory already exists")]
    MgrAlreadyExists,
    #[error("failed to create a workspace manager directory")]
    CreateMgrError(io::Error),
    #[error("failed to create a vars file")]
    CreateVarsFileError(io::Error),
}

pub fn print_error(error: Error) {
    let println = |msg: String| println!("{} {}", error_prefix(), msg);
    let _ = match error {
        Error::AppConfigError(e) => {
            println(e.to_string());
            match e {
                AppConfigError::AppInitError(e) => {
                    println(e.to_string());
                    match e {
                        AppInitError::AppConfDirCreateError(e) => {
                            println(e.to_string());
                        }
                        AppInitError::CheckAppConfPresenceError(e) => {
                            println(e.to_string());
                        }
                        AppInitError::DynConfInitError(e) => {
                            println(e.to_string());
                            match e {
                                DynConfInitError::CreateError(e) => {
                                    println(e.to_string());
                                }
                                DynConfInitError::ParseError(e) => {
                                    println(e.to_string());
                                }
                                DynConfInitError::WriteError(e) => {
                                    println(e.to_string());
                                }
                            }
                        }
                        AppInitError::InitAlreadyDone => {}
                        AppInitError::ShellHistInitError(e) => {
                            println(e.to_string());
                            match e {
                                ShellHistInitError::CreateError(e) => {
                                    println(e.to_string());
                                }
                            }
                        }
                    }
                }
                AppConfigError::ReadDynConfError(e) => {
                    println(e.to_string());
                    match e {
                        ReadDynConfError::OpenError(e) => {
                            println(e.to_string());
                        }
                        ReadDynConfError::ReadError(e) => {
                            println(e.to_string());
                        }
                        ReadDynConfError::ParseError(e) => {
                            println(e.to_string());
                        }
                    }
                }
            }
        }
        Error::UseCurrentDirAsWorkspaceError(e) => {
            println(e.to_string());
            match e {
                UseCurrentDirAsWorkspaceError::BeforeInitError => {}
                UseCurrentDirAsWorkspaceError::CheckMgrPresenceError(e) => {
                    println(e.to_string());
                }
                UseCurrentDirAsWorkspaceError::GetCurrentDirError(e) => {
                    println(e.to_string());
                }
                UseCurrentDirAsWorkspaceError::UpdateDynConfFileError(e) => {
                    println(e.to_string());
                    match e {
                        UpdateDynConfFileError::OpenError(e) => {
                            println(e.to_string());
                        }
                        UpdateDynConfFileError::ParseError(e) => {
                            println(e.to_string());
                        }
                        UpdateDynConfFileError::WriteError(e) => {
                            println(e.to_string());
                        }
                    }
                }
            }
        }
        Error::InitCurrentDirAsWorkspaceError(e) => {
            println(e.to_string());
            match e {
                InitCurrentDirAsWorkspaceError::CreateNewWorkspaceError(e) => {
                    println(e.to_string());
                    match e {
                        CreateNewWorkspaceError::CheckMgrPresenceError(e) => {
                            e.to_string();
                        }
                        CreateNewWorkspaceError::CreateMgrError(e) => {
                            e.to_string();
                        }
                        CreateNewWorkspaceError::CreateVarsFileError(e) => {
                            e.to_string();
                        }
                        CreateNewWorkspaceError::MgrAlreadyExists => {}
                    }
                }
                InitCurrentDirAsWorkspaceError::GetCurrentDirError(e) => {
                    println(e.to_string());
                }
                InitCurrentDirAsWorkspaceError::UseCurrentDirAsWorkspaceError(e) => {
                    println(e.to_string());
                    match e {
                        UseCurrentDirAsWorkspaceError::BeforeInitError => {}
                        UseCurrentDirAsWorkspaceError::CheckMgrPresenceError(e) => {
                            println(e.to_string());
                        }
                        UseCurrentDirAsWorkspaceError::GetCurrentDirError(e) => {
                            println(e.to_string());
                        }
                        UseCurrentDirAsWorkspaceError::UpdateDynConfFileError(e) => {
                            println(e.to_string());
                            match e {
                                UpdateDynConfFileError::OpenError(e) => {
                                    println(e.to_string());
                                }
                                UpdateDynConfFileError::ParseError(e) => {
                                    println(e.to_string());
                                }
                                UpdateDynConfFileError::WriteError(e) => {
                                    println(e.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
