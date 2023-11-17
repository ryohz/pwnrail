pub mod refer;
pub mod help;
pub mod modify;

pub fn commands() -> Vec<crate::shell::command::Command> {
    vec![
        crate::shell::command::Command::new("v", Box::new(help::help1)),
        crate::shell::command::Command::new("vh", Box::new(help::help1)),
        crate::shell::command::Command::new("vr", Box::new(refer::refer)),
        crate::shell::command::Command::new("vm", Box::new(modify::modify)),
    ]
}
