use std::io::{self, Read, Write};

use crate::output::{error_prefix, log_prefix};

pub fn commands() -> Vec<crate::shell::command::Command> {
    vec![
        crate::shell::command::Command::new("vr", Box::new(refer)),
        crate::shell::command::Command::new("vm", Box::new(modify)),
    ]
}

fn refer(args: String, app_conf: &crate::config::AppConfig) -> bool {
    false
}

fn modify(args: String, app_conf: &crate::config::AppConfig) -> bool {
    false
}
