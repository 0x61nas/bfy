use crate::interpreter::Interpreter;
use std::io::Write;

struct Repl {
    interpreter: Interpreter,
    history: Vec<String>,
}

impl Repl {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            history: Vec::new(),
        }
    }

    pub fn run(mut self) {
        loop {
            print!("\n> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();

            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to read input: {}", e);
                    std::process::exit(1);
                }
            }
            input = input.trim().to_string(); // Remove trailing newline

            self.history.push(input.clone()); // Save input to history

            if input.starts_with("!") {
                self.run_repl_cmd(input);
            } else {
                match self.interpreter.run(Some(input)) {
                    Ok(_) => {
                        info!("Successfully ran brainfuck source code from REPL");
                    }
                    Err(e) => {
                        error!("Failed to run brainfuck source code from REPL: {}", e);
                    }
                }
            }
        }
    }

    fn run_repl_cmd(&mut self, input: String) {
        let mut cmd = input.split_whitespace();
        match cmd.next() {
            Some(repl_cmd) => {
                match repl_cmd {
                    "fuck" => {
                        println!("Bye bye :D");
                        std::process::exit(0);
                    }
                    "array" | "a" => {
                        println!("Current array: {:?}", self.interpreter.cells);
                    }
                    "array_size" | "as" => {
                        println!("Current array size: {}", self.interpreter.array_size);
                    }
                    "pointer" | "p" => {
                        println!("Current pointer: {}", self.interpreter.pointer);
                    }
                    "history" | "h" => {
                        println!("History:");
                        for (i, cmd) in self.history.iter().enumerate() {
                            println!("{}: {}", i, cmd);
                        }
                    }
                    "save" | "s" => {
                        let file_name = cmd.next().unwrap_or("brainfuck_repl_history.bfr");

                        println!("Saving history to file: {file_name}");
                        match std::fs::write(file_name, self.history.join("\n")) {
                            Ok(_) => {
                                info!("Successfully saved history to file: {file_name}");
                            }
                            Err(e) => {
                                error!("Failed to save history to file: {}", e);
                            }
                        }
                    }
                    "load" | "l" => {
                        let file_name = cmd.next().unwrap_or("brainfuck_repl_history.bfr");

                        println!("Loading history from file: {file_name}");
                        match std::fs::read_to_string(file_name) {
                            Ok(history) => {
                                info!("Successfully loaded history from file: {file_name}");
                                self.history = history.split("\n").map(|s| s.to_string()).collect();

                                // Run all commands in history
                                for cmd in self.history.iter() {
                                    match self.interpreter.run(Some(cmd.clone())) {
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
                        println!("Resetting REPL");
                        self.interpreter.reset();
                    }
                    "help" => {
                        println!("!array, !a: print the current array");
                        println!("!array_size, !as: print the current array size");
                        println!("!pointer, !p: print the current pointer value");
                        println!("!history, !h: print the history of the commands");
                        println!("!save, !s: save the history to a file");
                        println!("!load, !l: load the history from a file");
                        println!("!reset, !r: reset the interpreter");
                        println!("!help: show this fu*king help message");
                        println!("!fuck: exit the REPL mode");
                    }
                    _ => println!("Unknown command: {}, type !help to show the help", input),
                }
            }
            None => {}
        }
    }
}

pub fn start(interpreter: Interpreter) {
    info!("Entering REPL mode");
    println!("Welcome to the brainfuck REPL mode! :)");
    println!(
        "Brainfuck interpreter v {}\nBy {}",
        clap::crate_version!(),
        clap::crate_authors!()
    );
    println!("Enter your brainfuck code and press enter to run it.");
    println!("Enter !fuck to exit :D");
    println!("Enter !help fuck to get more help");

    Repl::new(interpreter).run();
}
