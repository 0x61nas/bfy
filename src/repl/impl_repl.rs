use super::repl::Repl;
use crate::bf_interpreter::interpreter::Interpreter;
use crate::repl::repl::{COMMAND_PREFIX, HISTORY_FILE, PROMPT};
use colored::Colorize;
use console::Key;
use std::io::Write;

impl Repl {
    pub fn new(interpreter: Interpreter) -> Repl {
        Repl {
            term: interpreter.term.clone(),
            interpreter,
            history: Vec::new(),
            loop_body: String::new(),
            loop_depth: 0,
        }
    }

    // #[no_panic]
    pub fn run(mut self) -> Result<(), std::io::Error> {
        loop {
            self.print_prompt();

            std::io::stdout().flush()?;

            match self.read_input() {
                Ok(input) => {
                    let user_input = input.trim().to_string(); // Remove trailing newline

                    if !user_input.is_empty() && user_input.len() > 0 {
                        self.history.push(user_input.clone()); // Save input to history
                        self.process(user_input); // Process the input
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    fn print_prompt(&self) {
        print!(
            "{}",
            if self.loop_depth != 0 {
                "........ ".yellow()
            } else {
                PROMPT.to_string().truecolor(54, 76, 76)
            }
        );
    }

    fn read_input(&mut self) -> Result<String, std::io::Error> {
        let mut input = String::new();
        let mut rev_index = 0;

        loop {
            let key = self.term.read_key()?; // Read key from terminal

            match key {
                Key::ArrowUp => {
                    if !self.history.is_empty() && rev_index < self.history.len() {
                        let last = self
                            .history
                            .get(self.history.len() - 1 - rev_index)
                            .unwrap();
                        rev_index += 1;
                        self.term.clear_line()?;
                        self.print_prompt();
                        self.term.write_str(last)?;
                        input = last.clone();
                    }
                }
                Key::ArrowDown => {
                    if !self.history.is_empty() && rev_index > 0 {
                        let first = self.history.get(self.history.len() - rev_index).unwrap();
                        rev_index -= 1;
                        self.term.clear_line()?;
                        self.print_prompt();
                        self.term.write_str(first)?;
                        input = first.clone();
                    }
                }
                Key::Char(c) => {
                    self.term.write_str(&c.to_string())?;
                    input.push(c);
                }
                Key::Backspace => {
                    self.term.clear_line()?;
                    self.term.write_str(&input[0..input.len() - 1])?;
                    self.term.move_cursor_left(1)?;
                    input.pop();
                }
                Key::Enter => {
                    self.term.write_str("\n")?;
                    break;
                }
                _ => {}
            }
        }
        Ok(input)
    }

    pub fn process(&mut self, mut user_input: String) {
        user_input.chars().for_each(|ch| {
            if ch == '[' {
                self.loop_depth += 1;
            } else if ch == ']' {
                self.loop_depth -= 1;
            }
        });
        match user_input.find('[') {
            Some(index) if self.loop_depth != 0 && self.loop_body.is_empty() => {
                self.loop_body.push_str(&user_input[index..]);
                user_input = user_input[..index].to_string();
            }
            Some(_) if !self.loop_body.is_empty() => {
                self.loop_body.push_str(&user_input);
                return;
            }
            _ => {
                if user_input.contains(']') {
                    if self.loop_depth == 0 {
                        self.loop_body.push_str(&user_input);
                        user_input = self.loop_body.clone();
                        self.loop_body = String::new();
                    }
                }
                if self.loop_depth != 0 {
                    self.loop_body.push_str(&user_input);
                    return;
                }
            }
        }

        if user_input.is_empty() || user_input.len() == 0 {
            return;
        }

        if user_input.starts_with(COMMAND_PREFIX) {
            self.run_repl_cmd(user_input);
        } else {
            match self.interpreter.run(user_input) {
                Ok(_) => {
                    info!("Successfully ran brainfuck source code from REPL");
                }
                Err(e) => {
                    error!("Failed to run brainfuck source code from REPL: {}", e);
                }
            }
        }
    }

    fn run_repl_cmd(&mut self, user_input: String) {
        let mut cmd = user_input.split_whitespace();
        match cmd.next() {
            Some(repl_cmd) => {
                match repl_cmd.get(COMMAND_PREFIX.len()..).unwrap_or("") {
                    "fuck" => {
                        println!("{}", "Bye bye :D".green());
                        std::process::exit(0);
                    }
                    "array" | "a" => {
                        println!("{}", format!("Current array: {:?}", self.interpreter.cells));
                    }
                    "array_size" | "as" => {
                        println!(
                            "{}",
                            format!(
                                "Current array size: {}",
                                self.interpreter.cells.len().to_string().bold().green()
                            )
                        );
                    }
                    "pointer" | "p" => {
                        println!(
                            "{}",
                            format!(
                                "Current pointer: {}",
                                self.interpreter.pointer.to_string().bold().green()
                            )
                        );
                    }
                    "pointer_value" | "pv" => {
                        println!(
                            "Current pointer value: {} = \'{}\' (char)",
                            self.interpreter.cells[self.interpreter.pointer],
                            self.interpreter.cells[self.interpreter.pointer]
                                .to_char()
                                .unwrap_or_else(|_| '?')
                        );
                    }
                    "history" | "h" => {
                        println!("{}", "History:".underline().green());
                        for (i, cmd) in self.history.iter().enumerate() {
                            println!("{}", format!("{}: {}", i, cmd));
                        }
                    }
                    "save" | "s" => {
                        let file_name = cmd.next().unwrap_or(HISTORY_FILE);

                        println!(
                            "{}",
                            format!("Saving history to file: {file_name}").yellow()
                        );
                        match std::fs::write(file_name, self.history.join("\n")) {
                            Ok(_) => {
                                println!(
                                    "{}",
                                    format!("Successfully saved history to file: {file_name}")
                                        .green()
                                );
                            }
                            Err(e) => {
                                error!("Failed to save history to file: {}", e);
                            }
                        }
                    }
                    "load" | "l" => {
                        let file_name = cmd.next().unwrap_or(HISTORY_FILE);

                        println!(
                            "{}",
                            format!("Loading history from file: {file_name}").yellow()
                        );
                        match std::fs::read_to_string(file_name) {
                            Ok(history) => {
                                println!(
                                    "{}",
                                    format!("Successfully loaded history from file: {file_name}")
                                        .green()
                                );
                                self.history = history.split("\n").map(|s| s.to_string()).collect();

                                // Run all commands in history
                                for cmd in self.history.iter() {
                                    match self.interpreter.run(cmd.clone()) {
                                        Ok(_) => {
                                            info!(
                                                "Successfully ran brainfuck source code from REPL"
                                            );
                                        }
                                        Err(e) => {
                                            error!(
                                                "Failed to run brainfuck source code from REPL: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to load history from file: {}", e);
                            }
                        }
                    }
                    "reset" | "r" => {
                        println!("{}", "Resetting REPL".truecolor(56, 33, 102));
                        self.interpreter.reset();
                        self.history = Vec::new();
                    }
                    "help" => {
                        println!(
                            "!array, !a: print the current array\n\
                        !array_size, !as: print the current array size\n\
                        !pointer, !p: print the current pointer\n\
                        !pointer_value, !pv: print the current pointer value\n\
                        !history, !h: print the REPL history\n\
                        !save, !s: save the REPL history to a file\n\
                        !load, !l: load the REPL history from a file\n\
                        !reset, !r: reset the REPL\n\
                        !help: print this help message\n\
                        !fuck: exit the REPL"
                        );
                    }
                    _ => println!(
                        "{}",
                        format!(
                            "Unknown command: {}, type {} to show the help",
                            user_input,
                            (COMMAND_PREFIX.to_string() + "help").green()
                        )
                        .red()
                    ),
                }
            }
            None => {}
        }
    }
}
