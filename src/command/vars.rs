use std::io::{self, Read, Write};

use crate::output::{error_prefix, green, log_prefix};

pub fn commands() -> Vec<crate::shell::command::Command> {
    vec![
        crate::shell::command::Command::new("v", Box::new(help1)),
        crate::shell::command::Command::new("vh", Box::new(help1)),
        crate::shell::command::Command::new("vr", Box::new(refer)),
        crate::shell::command::Command::new("vm", Box::new(modify)),
    ]
}

fn help1(args: String, app_conf: &crate::config::AppConfig) -> bool {
    println!("{}ars", green("V"));
    println!("  refer or modify variables like an ip address.");
    println!("Commands:");
    println!("\tv\tprint this screen.");
    println!("\tvh\tprint this screen.");
    println!("\tvhh\tprint verbose help about Vars.");
    println!(
        "\tvr\trefer the variables by a json query like this: \"vr ip\", \"vr creds[0].password\".",
    );
    println!(
        "\tvm\tmodify the variables by a json query. when you want to register the ip adress, you can do it with this: \"vm ip 0.0.0.0\" for example."
    );
    false
}

fn help2(args: String, app_conf: &crate::config::AppConfig) -> bool {
    false
}

fn refer(args: String, app_conf: &crate::config::AppConfig) -> bool {
    false
}

fn modify(args: String, app_conf: &crate::config::AppConfig) -> bool {
    false
}
