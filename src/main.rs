use command::command::start_shell;
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
    let app_conf = match config::AppConfig::new() {
        Ok(conf) => conf,
        Err(e) => {
            println!("{} {}", error_prefix(), e.to_string());
            return;
        }
    };

    start_shell(app_conf).await;
}
