use std::io::Write;
use crate::interpreter::Interpreter;

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
                Ok(_) => {},
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
                    Err((e, _)) => {
                        error!("Failed to run brainfuck source code from REPL: {}", e);
                    }
                }
            }
        }
    }

    fn run_repl_cmd(&mut self, input: String) {
        match input.trim().get(1..).unwrap() {
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
                /// TODO: Use custom name for file
                println!("Saving history to file: history.bfr");
                match std::fs::write("history.bfr", self.history.join("\n")) {
                    Ok(_) => {
                        info!("Successfully saved history to file: history.bfr");
                    }
                    Err(e) => {
                        error!("Failed to save history to file: {}", e);
                    }
                }
            },
            "reset" | "r" => {
                println!("Resetting REPL");
                self.interpreter.reset();
            },
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
            _ => println!("Unknown command: {}, type !help to show the help", input)
        }
    }
}

pub fn start(interpreter: Interpreter) {
    info!("Entering REPL mode");
    println!("Welcome to the brainfuck REPL mode! :)");
    println!("Brainfuck interpreter v {}\nBy {}",
             clap::crate_version!(), clap::crate_authors!());
    println!("Enter your brainfuck code and press enter to run it.");
    println!("Enter !fuck to exit :D");
    println!("Enter !help fuck to get more help");

    Repl::new(interpreter)
        .run();
}
