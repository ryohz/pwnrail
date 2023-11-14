pub fn error_prefix() -> String {
    format!("[{}]", red("err"))
}

pub fn log_prefix() -> String {
    format!("[{}]", green("info"))
}

pub fn red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}
