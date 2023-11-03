mod console;
mod error;
mod util;

// use std::{fs, io, path::PathBuf};

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

#[derive(Subcommand, Debug)]
enum SubCommands {
    Console(ConsoleArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Console(args) => {
            console::console::start().await;
        }
    }
}
