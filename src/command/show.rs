pub fn commands() -> Vec<crate::shell::command::Command> {
    vec![crate::shell::command::Command::new(
        "scw",
        Box::new(crate::command::show::show_current_workspace),
    )]
}

fn show_current_workspace(_args: String, app_conf: &mut crate::config::AppConfig) -> bool {
    println!("{}", app_conf.dyn_conf.current_workspace);
    false
}
