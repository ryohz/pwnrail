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
    let is_initialized = match config::is_app_initialized() {
        Ok(b) => b,
        Err(e) => {
            println!(
                "{} failed to get if app has been initialized: {}",
                error_prefix(),
                e.to_string()
            );
            return;
        }
    };
    if is_initialized {
        let _ = match config::app_init() {
            Ok(_) => (),
            Err(e) => match e {
                AppInitError::InitAlreadyDone => (),
                _ => {
                    println!("{} failed to init app: {}", error_prefix(), e.to_string());
                    return;
                }
            },
        };
    }
}
