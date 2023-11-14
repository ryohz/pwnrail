pub struct Command {
    pub name: String,
    // variable to store function that will be called when the paired name is used on the prompt
    // when it's called, arguments of the command is passwd via a string argument.
    pub func: Box<dyn Fn(String) -> super::types::IsError>,
}

impl Command {
    pub fn new(name: &str, func: Box<dyn Fn(String) -> super::types::IsError>) -> Self {
        let name = name.to_string();
        Command { name, func }
    }
}

pub fn builtins() -> Vec<Command> {
    vec![Command::new("h", Box::new(hello))]
}

pub fn hello(arg: String) -> super::types::IsError {
    println!("hello");
    false
}
