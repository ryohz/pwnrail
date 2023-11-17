use std::env;

use crate::{
    config,
    error::{self, Error},
    output::{error_prefix, green, red},
    shell,
};

pub fn commands() -> Vec<shell::command::Command> {
    let mut commands = vec![
        crate::shell::command::Command::new("use", Box::new(use_)),
        crate::shell::command::Command::new("init", Box::new(init)),
    ];
    let vars_cmmands = super::vars::commands();
    commands.extend(vars_cmmands);
    commands
}

pub async fn start_shell(app_conf: &mut config::AppConfig) {
    let mut prompt = shell::shell::Shell::new(
        Some(commands()),
        Some(green("grv>")),
        Some(red("grv>")),
        app_conf,
    );
    prompt.start().await;
}

// カレントディレクトリをワークスペースとして初期化するコマンド関数
fn init(_args: String, app_conf: &mut crate::config::AppConfig) -> bool {
    let _ = match app_conf.init_current_directory_as_workspace() {
        Ok(_) => (),
        Err(e) => {
            println!(
                "{} failed to init current directory as workspace",
                error_prefix()
            );
            error::print_error(Error::InitCurrentDirAsWorkspaceError(e));
            println!();
            return true;
        }
    };
    false
}

// ワークスペースの場所をカレントディレクトリに変更するコマンド関数
fn use_(_args: String, app_conf: &mut crate::config::AppConfig) -> bool {
    let _ = match app_conf.use_current_dir_as_workspace() {
        Ok(_) => (),
        Err(e) => {
            println!(
                "{} failed to use current directory as workspace",
                error_prefix()
            );
            error::print_error(Error::UseCurrentDirAsWorkspaceError(e));
            println!();
            return true;
        }
    };
    false
}
