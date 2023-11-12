pub mod envinfo {
    use dirs;
    use std::fs::File;
    use std::io::{self, BufRead, Write};
    use std::path::PathBuf;
    use std::{env, fs};

    use crate::error::Error;

    pub const GLOBAL_CONFIG_DIR_NAME: &str = ".pentenv";
    pub const CURRENT_WORKSPACE_FILE: &str = "current";
    pub const ENV_DIR_NAME: &str = ".ptv";
    pub const VARS_FILE_NAME: &str = "vars.json";
    pub const HISTORY_FILE_NAME: &str = "history";

    // this func initializes current directory
    // it does it directory, so the other functions have to handle
    // something(whether old env files already exists and etc) before it's called
    pub fn init() -> Result<(), Error> {
        let current_path = match env::current_dir() {
            Ok(path) => path,
            Err(e) => return Err(Error::IoError(e)),
        };
        // base path
        // creating env directory
        // if it already exists, it returns error
        // so after this handle, precense of env directory is premise
        let base_path = current_path.join(ENV_DIR_NAME);
        if base_path.exists() {
            let msg = format!("{} already exists.", ENV_DIR_NAME);
            return Err(Error::AlreadyExists(msg));
        }
        fs::create_dir(&base_path)?;
        // vars.json
        let vars_path = base_path.join(VARS_FILE_NAME);
        let mut vars_file = fs::File::create(&vars_path)?;
        let _ = write!(vars_file, "{{}}")?;
        // global config
        let gconfig_dir_path = gconfig_dir_path()?;
        let current_ws_mgr_path = current_ws_mgr_path()?;
        if fs::metadata(&gconfig_dir_path).is_err() {
            fs::create_dir_all(&gconfig_dir_path)?;
        }
        update_current_ws(&base_path);
        // history
        let history_path = history_file_path()?;
        let _ = match fs::metadata(&history_path) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    fs::File::create(&history_path)?;
                }
                return Err(Error::IoError(e));
            }
        };
        Ok(())
    }

    pub fn home_path() -> Result<PathBuf, Error> {
        match dirs::home_dir() {
            Some(p) => Ok(p),
            None => return Err(Error::NoHomeDir),
        }
    }

    pub fn gconfig_dir_path() -> Result<PathBuf, Error> {
        let home = home_path()?;
        Ok(home.join(GLOBAL_CONFIG_DIR_NAME))
    }

    pub fn current_ws_mgr_path() -> Result<PathBuf, Error> {
        let g = gconfig_dir_path()?;
        Ok(g.join(CURRENT_WORKSPACE_FILE))
    }

    pub fn current_ws_path() -> Result<PathBuf, Error> {
        let path = current_ws_mgr_path()?;
        let mut buf = io::BufReader::new(match fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    return Err(Error::BeforeInitEnv);
                }
                return Err(Error::IoError(e));
            }
        });
        let mut current_path_string = String::new();
        let _ = match buf.read_line(&mut current_path_string) {
            Ok(_) => (),
            Err(e) => return Err(Error::IoError(e)),
        };
        current_path_string = current_path_string.replace('\n', "");
        Ok(PathBuf::from(current_path_string))
    }

    pub fn update_current_ws(path: &PathBuf) -> Result<(), Error> {
        let next_path = path.join(ENV_DIR_NAME);
        let _ = fs::metadata(&next_path)?;
        let _ = fs::metadata(current_ws_mgr_path()?)?;
        let mgr = fs::File::create(current_ws_mgr_path()?)?;
        let mut writer = io::BufWriter::new(mgr);
        writer.write(&next_path.to_str().unwrap().as_bytes())?;
        Ok(())
    }

    pub fn vars_file_write() -> Result<File, Error> {
        let base_path = current_ws_path()?;
        let path = base_path.join(VARS_FILE_NAME);
        match File::create(path) {
            Ok(f) => Ok(f),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Err(Error::BeforeInitEnv)
                } else {
                    Err(Error::IoError(e))
                }
            }
        }
    }

    pub fn vars_file_read() -> Result<File, Error> {
        let base_path = current_ws_path()?;
        let path = base_path.join(VARS_FILE_NAME);
        match File::open(&path) {
            Ok(f) => Ok(f),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Err(Error::BeforeInitEnv)
                } else {
                    Err(Error::IoError(e))
                }
            }
        }
    }

    pub fn history_file_path() -> Result<PathBuf, Error> {
        let path = gconfig_dir_path()?;
        Ok(path.join(HISTORY_FILE_NAME))
    }

    pub fn ginit() -> Result<(), Error> {
        // config dir
        let gconfig_path = gconfig_dir_path()?;
        create_dir_if_not_exist(&gconfig_path)?;
        // current workspace manager
        let current_ws_mgr_path = current_ws_mgr_path()?;
        create_file_if_not_exist(&current_ws_mgr_path)?;
        // history
        let history_path = history_file_path()?;
        create_file_if_not_exist(&history_path)?;
        Ok(())
    }

    fn create_file_if_not_exist(path: &PathBuf) -> Result<(), Error> {
        let _ = match fs::metadata(path) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    fs::File::create(path)?;
                    return Ok(());
                }
                return Err(Error::IoError(e));
            }
        };
        Ok(())
    }

    fn create_dir_if_not_exist(path: &PathBuf) -> Result<(), Error> {
        let _ = match fs::metadata(path) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    fs::create_dir(path)?;
                    return Ok(());
                }
                return Err(Error::IoError(e));
            }
        };
        Ok(())
    }
}

pub mod clipboard {

    use std::{io::Write, process::Command};

    use crate::error::Error;

    pub fn copy(text: &String) -> Result<(), Error> {
        Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .arg("-i")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute command")
            .stdin
            .expect("Failed to open stdin")
            .write_all(text.as_bytes())?;
        Ok(())
    }
}
