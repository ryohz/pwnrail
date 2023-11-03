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
        let mut current_ws_mgr_buf =
            io::BufWriter::new(match fs::File::open(&current_ws_mgr_path) {
                Ok(f) => f,
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        fs::File::create(&current_ws_mgr_path)?
                    } else {
                        return Err(Error::IoError(e));
                    }
                }
            });
        let base_path_str = base_path.to_str().unwrap();
        let _ = current_ws_mgr_buf.write(base_path_str.as_bytes())?;
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
                    return Err(Error::BeforeInit);
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

    pub fn vars_file_write() -> Result<File, Error> {
        let base_path = current_ws_path()?;
        let path = base_path.join(VARS_FILE_NAME);
        match File::create(path) {
            Ok(f) => Ok(f),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Err(Error::BeforeInit)
                } else {
                    Err(Error::IoError(e))
                }
            }
        }
    }

    pub fn vars_file_read() -> Result<File, Error> {
        let base_path = current_ws_path()?;
        let path = base_path.join(VARS_FILE_NAME);
        match File::open(path) {
            Ok(f) => Ok(f),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Err(Error::BeforeInit)
                } else {
                    Err(Error::IoError(e))
                }
            }
        }
    }
}

pub mod clipboard {

    use crate::error::Error;
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;

    pub fn copy(text: &String) -> Result<(), Error> {
        // error handling is under conducting
        let mut ctx: ClipboardContext = match ClipboardProvider::new() {
            Ok(c) => c,
            Err(e) => return Err(Error::CopyToClipboardError(e.to_string())),
        };
        let _ = match ctx.set_contents(text.to_owned()) {
            Ok(_) => (),
            Err(e) => return Err(Error::CopyToClipboardError(e.to_string())),
        };
        Ok(())
    }
}
