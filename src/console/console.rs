use std::{env, fs, io};

use crate::{error::Error, output::error_prefix, shell};

// console
// |- init initialize env. create .<projectname> directory.
// |       if it already exists, console ask user whether he wants to clean current environment.
// |       i.e. remove .<projectname> or clean command.
// |- clean remove env directory i.e .<projectname>.
// |- v store variable (for example ip, hosts, subdomain...) as json.
// |- t run specified tool like nmap, rustscan, gobuster and etc.

pub fn commands() -> Vec<shell::command::Command> {
    vec![
        shell::command::Command::new("v", Box::new(super::vars::vars)),
        shell::command::Command::new("init", Box::new(init)),
        shell::command::Command::new("use", Box::new(use_)),
        shell::command::Command::new("clean", Box::new(clean)),
    ]
}

pub async fn start() {
    let history_path = match super::util::envinfo::history_file_path() {
        Ok(p) => p,
        Err(e) => {
            println!(
                "{} failed to find history file: {}",
                error_prefix(),
                e.to_string()
            );
            return;
        }
    };
    let mut prompt =
        shell::prompt::Prompt::new(Some(commands()), Some("(pentenv) $ "), history_path);

    prompt.start().await;
}

fn init(_: String) -> shell::types::IsError {
    let result = super::util::envinfo::init();
    match result {
        Ok(_) => shell::types::IsError::from(false),
        Err(e) => match e {
            Error::AlreadyExists(_) => {
                output::errorln!("failed to initialize: \n\t{}", e.to_string());
                shell::types::IsError::from(true)
            }
            _ => {
                output::errorln!("failed to initialize: \n\t{}", e.to_string());
                shell::types::IsError::from(true)
            }
        },
    }
}

fn clean(_: String) -> shell::types::IsError {
    let base_path = match super::util::envinfo::current_ws_path() {
        Ok(p) => p,
        Err(e) => {
            output::errorln!("failed to clean current workspace.");
            output::errorln!("{}", e.to_string());
            return true;
        }
    };
    shell::console::output::println!("removing {}", &base_path.to_str().unwrap());
    match fs::remove_dir_all(&base_path) {
        Ok(_) => {
            output::println!("removed {}", &base_path.to_str().unwrap());
            output::println!("the environment is cleaned successfully");
            false
        }
        Err(e) => {
            output::errorln!("failed to remove {}", &base_path.to_str().unwrap());
            output::errorln!("{}", e.to_string());
            output::errorln!("failed to clean environment");
            true
        }
    }
}

fn use_(_arg: String) -> shell::types::IsError {
    // does .ptv exists? -no-> error
    let current_path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            println!("failed to get current dir: {}", e.to_string());
            return true;
        }
    };
    let _ = match super::util::envinfo::update_current_ws(&current_path) {
        Ok(_) => (),
        Err(e) => {
            println!("failed to update current workspace: {}", e.to_string());
        }
    };
    false
}
