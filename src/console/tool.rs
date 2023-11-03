use clap::{error::ErrorKind as ClapErrorKind, Parser, Subcommand};
use rskai::console::output;

use crate::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ToolArgs {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    List(ListArgs),
}

#[derive(Parser, Debug)]
struct ListArgs {
    path: String,
    value: String,
    #[arg(short)]
    show: bool,
}

pub fn tool(argument: String) -> rskai::types::IsError {
    let mut args_iter = vec![""];
    args_iter.extend(&argument.split_whitespace().collect::<Vec<&str>>());

    let args = match ToolArgs::try_parse_from(args_iter) {
        Ok(args) => args,
        Err(e) => {
            let kind = e.kind();
            if kind == ClapErrorKind::MissingRequiredArgument
                || kind == ClapErrorKind::MissingSubcommand
                || kind == ClapErrorKind::DisplayHelp
                || kind == ClapErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
                || kind == ClapErrorKind::DisplayVersion
            {
                output::print!("{}", e.to_string());
                return false;
            }
            output::errorln!("failed to parse arguments");
            output::errorln!("{}", e.to_string());
            return true;
        }
    };

    match &args.subcommand {
        SubCommands::List(args) => {
            return true;
        }
        _ => return true,
    }
}
