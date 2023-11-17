use command::command::start_shell;

mod command;
mod config;
mod error;
mod json;
mod output;
mod shell;

#[tokio::main]
async fn main() {
    // 初期設定
    let mut app_conf = match config::AppConfig::new() {
        Ok(conf) => conf,
        Err(e) => {
            crate::error::print_error(crate::error::Error::AppConfigError(e));
            println!();
            return;
        }
    };

    start_shell(&mut app_conf).await;
}
