use std::io::{self, Read, Write};

use clap::Parser;

use crate::{
    json::{self, error::JsonQueryError},
    output::{blue, error_prefix, green, log_prefix},
};

// vr (refer)では独自のjsonクエリで場所を指定してその場所にある値をプリントする。
// copyフラグを追加することでその値を自動でクリップボードにコピーできる
#[derive(Parser, Debug)]
struct RefArgs {
    path: String,
    #[arg(short)]
    copy: bool,
}

pub fn refer(args_: String, app_conf: &mut crate::config::AppConfig) -> bool {
    let err_msg = || {
        println!("{} vars reference error", error_prefix());
    };
    let mut args_iter = vec![""];
    args_iter.extend(&args_.split_whitespace().collect::<Vec<&str>>());
    let args = match RefArgs::try_parse_from(&args_iter) {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e.to_string());
            return true;
        }
    };
    let json_path = args.path;
    let path = app_conf.dyn_conf.to_workspace().vars_path;
    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            err_msg();
            println!("{} failed to open the file", error_prefix(),);
            println!("{} {}", error_prefix(), e.to_string());
            return true;
        }
    };
    let mut json_buf = String::new();
    let mut reader = io::BufReader::new(file);
    let _ = match reader.read_to_string(&mut json_buf) {
        Ok(_) => (),
        Err(e) => {
            err_msg();
            println!("{} failed to read the file", error_prefix(),);
            println!("{} {}", error_prefix(), e.to_string());
            return true;
        }
    };

    let mut j = json::json::Json::new(&json_buf);
    let result = match j.refer(&json_path) {
        Ok(r) => r,
        Err(e) => match e {
            JsonQueryError::NotFound => {
                println!("not found");
                return false;
            }
            _ => {
                err_msg();
                println!("{} failed to refer the value", error_prefix(),);
                println!("{} {}", error_prefix(), e.to_string());
                return true;
            }
        },
    };
    println!("{}", result);
    false
}
