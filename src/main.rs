use error::AppInitError;
use output::error_prefix;

mod command;
mod config;
mod error;
mod output;
mod shell;

#[tokio::main]
async fn main() {
    // 初期設定
    let app_config = match config::AppConfig::new() {
        Ok(conf) => conf,
        Err(e) => {
            println!("{} {}", error_prefix(), e.to_string());
            return;
        }
    };
}
