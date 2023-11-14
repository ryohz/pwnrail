use std::path::PathBuf;

// v -> vars
// vr -r-> vars
// vm -m-> vars

pub struct Prompt {
    pub commands: Vec<super::command::Command>,
    pub prompt_prefix: String,
    pub prev_state: bool,
    pub history_path: PathBuf,
}

impl Prompt {
    pub fn new(
        commands_: Option<Vec<super::command::Command>>,
        prompt_prefix_: Option<&str>,
        history_path: PathBuf,
    ) -> Self {
        let mut commands = super::command::builtins();
        let mut prompt_prefix = "rshell> ".to_string();
        let prev_state = false;

        if let Some(commands_) = commands_ {
            for cmd in commands_ {
                commands.push(cmd);
            }
        }
        if prompt_prefix_.is_some() {
            prompt_prefix = prompt_prefix_.unwrap().to_string();
        }

        Self {
            commands,
            prompt_prefix,
            prev_state,
            history_path,
        }
    }
    // entrypoint of interactive shell
    // this function accept user input and give the arguments to vary functions
    pub async fn start(&mut self) {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        let _ = rl.load_history(&self.history_path);
        loop {
            let readline = rl.readline(&self.prompt_prefix);
            let mut input = String::new();

            match readline {
                Ok(line) => {
                    input = line;
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    continue;
                }
                Err(rustyline::error::ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }

            let _ = rl.add_history_entry(input.as_str());

            if input == "exit" {
                break;
            }

            let raw_command: Vec<String> = input
                .split_whitespace()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let command_len = raw_command.len();

            if command_len == 0 {
                continue;
            }

            if command_len == 1 {
                let name = &raw_command[0];
                let state = self.execute_command(name, None);
                self.prev_state = state;
                continue;
            }

            if command_len > 1 {
                let name = &raw_command[0];
                let args = &raw_command[1..command_len].join(" ");
                let state = self.execute_command(name, Some(args));
                self.prev_state = state;
                continue;
            }
        }
        let _ = rl.save_history(&self.history_path);
    }

    fn execute_command(&self, name: &String, args: Option<&String>) -> super::types::IsError {
        let cmd_lct = self.search_command(name);
        match cmd_lct {
            Some(lct) => {
                let func = &self.commands[lct].func;
                if args.is_some() {
                    func(args.unwrap().to_string())
                } else {
                    func("".to_string())
                }
            }
            None => {
                println!("command {} is not found!", name);
                true
            }
        }
    }

    fn search_command(&self, name: &String) -> Option<usize> {
        for n in 0..self.commands.len() {
            if &self.commands[n].name == name {
                return Some(n);
            }
        }
        None
    }
}
