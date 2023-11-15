use crate::{config, output::error_prefix, shell};

pub fn commands() -> Vec<shell::command::Command> {
    let mut commands = vec![];
    let vars_cmmands = super::vars::commands();
    commands.extend(vars_cmmands);
    commands
}

pub async fn start_shell(app_conf: config::AppConfig) {
    let mut prompt = shell::shell::Shell::new(Some(commands()), Some("pentenv> "), &app_conf);
    prompt.start().await;
}
