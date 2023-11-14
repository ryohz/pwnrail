mod console;
mod error;
mod output;
mod shell;
mod util;

// use std::{fs, io, path::PathBuf};

use crate::output::error_prefix;
use std::env;

use clap::{Args, Parser, Subcommand};

// console
// |- init initialize env. create .<projectname> directory.
// |       if it already exists, console ask user whether he wants to clean current environment.
// |       i.e. remove .<projectname> or clean command.
// |- clean remove env directory i.e .<projectname>
// |-
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Args, Debug)]
struct ConsoleArgs {}

#[derive(Args, Debug)]
struct RunArgs {
    // #[arg(num_args(0..))]
    // direct: Option<String>,
    command: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Console(ConsoleArgs),
    Run(RunArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match util::envinfo::ginit() {
        Ok(_) => (),
        Err(e) => {
            println!(
                "{} failed to init application: {}",
                error_prefix(),
                e.to_string()
            );
        }
    }

    match cli.subcommand {
        SubCommands::Console(_args) => {
            console::console::start().await;
        }
        SubCommands::Run(args) => {
            let commands = crate::console::console::commands();
            if args.command.len() == 0 {
                println!("{} no command is given.", error_prefix());
                return;
            }
            let command_name = args.command.get(0).unwrap();
            let command_args = match args.command.get(1..) {
                Some(a) => a.join(" "),
                None => "".to_string(),
            };
            for command in commands {
                if command.name == command_name.to_owned() {
                    let func = command.func;
                    func(command_args);
                    return;
                }
            }
        }
    }
}
