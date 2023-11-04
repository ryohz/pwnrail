use std::io::{self, Read, Write};

use crate::util;

use super::error::Error;
use clap::error::ErrorKind as ClapErrorKind;
use clap::{Parser, Subcommand};
use rskai::console::output;

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

pub fn vars(argument: String) -> rskai::types::IsError {
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
                output::print!("{}", e.to_string());
                return false;
            }
            output::errorln!("failed to parse arguments");
            output::errorln!("{}", e.to_string());
            return true;
        }
    };
    match &args.subcommand {
        SubCommands::M(args) => match modify(args) {
            Ok(_) => return false,
            Err(e) => {
                output::errorln!("error occurred during modifying vars.");
                output::errorln!("{}", e.to_string());
                return true;
            }
        },
        SubCommands::R(args) => match refer(args) {
            Ok(_) => return false,
            Err(e) => {
                output::errorln!("error occurred during refering vars.");
                output::errorln!("{}", e.to_string());
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
                output::println!("not found: {}", e.to_string());
                return Ok(());
            }
            _ => return Err(Error::RjqlERror(e)),
        },
    };
    println!("{}", result);
    if args.copy {
        util::clipboard::copy(&result)?;
        output::println!("above value is copied to your clipboard!");
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
        output::println!("previous");
        output::println!("{}", j.data);
    }
    j.modify(dest, value)?;
    if args.show {
        output::println!("now");
        output::println!("{}", j.data);
    }

    let vars_file_w = util::envinfo::vars_file_write()?;
    let mut writer = io::BufWriter::new(vars_file_w);
    writer.write_all(j.data.to_string().as_bytes())?;

    Ok(())
}
