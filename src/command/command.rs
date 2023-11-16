use std::env;

use crate::{
    config,
    output::{error_prefix, green, red},
    shell,
};

pub fn commands() -> Vec<shell::command::Command> {
    let mut commands = vec![crate::shell::command::Command::new("init", Box::new(init))];
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

fn init(_args: String, app_conf: &mut crate::config::AppConfig) -> bool {
    let current_dir_path = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            println!("{} failed to init current directory", error_prefix());
            crate::error::print_error(crate::error::Error::GetCurrentDirectory(e));
            println!();
            return true;
        }
    };
    app_conf.dyn_conf.current_workspace = current_dir_path.to_str().unwrap().to_string();
    let _ = match app_conf.update_dyn_conf() {
        Ok(_) => (),
        Err(e) => {
            println!("{} failed to init current directory", error_prefix());
            crate::error::print_error(crate::error::Error::UpdateDynConfError(e));
            println!();
            return true;
        }
    };
    false
}
