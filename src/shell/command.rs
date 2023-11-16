pub struct Command {
    pub name: String,
    // variable to store function that will be called when the paired name is used on the prompt
    // when it's called, arguments of the command is passed via a string argument.
    pub func: Box<dyn Fn(String, &mut crate::config::AppConfig) -> super::types::IsError>,
}

impl Command {
    pub fn new(
        name: &str,
        func: Box<dyn Fn(String, &mut crate::config::AppConfig) -> super::types::IsError>,
    ) -> Self {
        let name = name.to_string();
        Command { name, func }
    }
}

pub fn builtins() -> Vec<Command> {
    vec![Command::new("h", Box::new(hello))]
}

pub fn hello(arg: String, app_conf: &mut crate::config::AppConfig) -> super::types::IsError {
    println!("hello");
    false
}
