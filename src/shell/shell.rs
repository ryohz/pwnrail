use std::path::PathBuf;

// v -> vars
// vr -r-> vars
// vm -m-> vars

pub struct Shell<'a> {
    pub commands: Vec<super::command::Command>,
    pub prompt: String,
    pub prev_state: bool,
    pub app_conf: &'a crate::config::AppConfig,
}

impl<'a> Shell<'a> {
    pub fn new(
        commands_: Option<Vec<super::command::Command>>,
        prompt_: Option<&str>,
        app_conf: &'a crate::config::AppConfig,
    ) -> Self {
        let mut commands = super::command::builtins();
        let mut prompt = "rshell> ".to_string();
        let prev_state = false;

        if let Some(commands_) = commands_ {
            for cmd in commands_ {
                commands.push(cmd);
            }
        }
        if prompt_.is_some() {
            prompt = prompt_.unwrap().to_string();
        }

        Self {
            commands,
            prompt,
            prev_state,
            app_conf,
        }
    }
    // entrypoint of interactive shell
    // this function accept user input and give the arguments to vary functions
    pub async fn start(&mut self) {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        let _ = rl.load_history(&self.app_conf.shell_hist_path);
        loop {
            let readline = rl.readline(&self.prompt);
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
        let _ = rl.save_history(&self.app_conf.shell_hist_path);
    }

    fn execute_command(&self, name: &String, args: Option<&String>) -> super::types::IsError {
        let cmd_lct = self.search_command(name);
        match cmd_lct {
            Some(lct) => {
                let func = &self.commands[lct].func;
                if args.is_some() {
                    func(args.unwrap().to_string(), self.app_conf)
                } else {
                    func("".to_string(), self.app_conf)
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
