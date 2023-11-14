use std::io::{self, Read, Write};

use crate::output::{error_prefix, log_prefix};
use crate::util;

use super::error::Error;
use clap::error::ErrorKind as ClapErrorKind;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct VarsArgs {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    M(ModArgs),
    R(RefArgs),
}

#[derive(Parser, Debug)]
struct ModArgs {
    path: String,
    value: String,
    #[arg(short)]
    show: bool,
}

#[derive(Parser, Debug)]
struct RefArgs {
    path: String,
    #[arg(short)]
    copy: bool,
}

pub fn vars(argument: String) -> crate::shell::types::IsError {
    let mut args_iter = vec![""];
    args_iter.extend(&argument.split_whitespace().collect::<Vec<&str>>());
    let args = match VarsArgs::try_parse_from(&args_iter) {
        Ok(args) => args,
        Err(e) => {
            let kind = e.kind();
            if kind == ClapErrorKind::MissingRequiredArgument
                || kind == ClapErrorKind::MissingSubcommand
                || kind == ClapErrorKind::DisplayHelp
                || kind == ClapErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
                || kind == ClapErrorKind::DisplayVersion
            {
                println!("{}", e.to_string());
                return false;
            }
            println!("{} failed to parse arguments", error_prefix());
            println!("{} {}", error_prefix(), e.to_string());
            return true;
        }
    };
    match &args.subcommand {
        SubCommands::M(args) => match modify(args) {
            Ok(_) => return false,
            Err(e) => {
                println!("{} error occurred during modifying vars.", error_prefix());
                println!("{} {}", error_prefix(), e.to_string());
                return true;
            }
        },
        SubCommands::R(args) => match refer(args) {
            Ok(_) => return false,
            Err(e) => {
                println!("{} error occurred during refering vars.", error_prefix());
                println!("{} {}", error_prefix(), e.to_string());
                return true;
            }
        },
    }
}

fn refer(args: &RefArgs) -> Result<(), Error> {
    let path = &args.path;

    let vars_file_r = util::envinfo::vars_file_read()?;
    let mut reader = io::BufReader::new(vars_file_r);

    let mut json_str = String::new();
    reader.read_to_string(&mut json_str)?;
    let mut j = rjql::json::Json::new(&json_str);

    let result = match j.refer(path) {
        Ok(r) => r,
        Err(e) => match e {
            rjql::error::Error::NotFound => {
                println!("{} not found: {}", error_prefix(), e.to_string());
                return Ok(());
            }
            _ => return Err(Error::RjqlERror(e)),
        },
    };
    println!("{}", result);
    if args.copy {
        util::clipboard::copy(&result)?;
        println!("{} above value is copied to your clipboard!", log_prefix());
    }
    Ok(())
}

fn modify(args: &ModArgs) -> Result<(), Error> {
    let value = &args.value;
    let dest = &args.path;

    let vars_file_r = util::envinfo::vars_file_read()?;
    let mut reader = io::BufReader::new(vars_file_r);

    let mut json_str = String::new();
    reader.read_to_string(&mut json_str)?;
    let mut j = rjql::json::Json::new(&json_str);

    if args.show {
        println!("{} previous:", log_prefix());
        println!("{} {}", log_prefix(), j.data);
    }
    j.modify(dest, value)?;
    if args.show {
        println!("{} current:", log_prefix());
        println!("{} {}", log_prefix(), j.data);
    }

    let vars_file_w = util::envinfo::vars_file_write()?;
    let mut writer = io::BufWriter::new(vars_file_w);
    writer.write_all(j.data.to_string().as_bytes())?;

    Ok(())
}
