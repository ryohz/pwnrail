use std::{env, fs, io};

use crate::{error::Error, output::error_prefix, shell};

pub fn commands() -> Vec<shell::command::Command> {
    let mut commands = vec![
        shell::command::Command::new("use", Box::new(use_)),
        shell::command::Command::new("clean", Box::new(clean)),
    ];
    let vars_cmmands = super::vars::commands();
    commands.extend(vars_cmmands);
    commands
}

pub async fn start_shell() {
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
