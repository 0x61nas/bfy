use crate::bf_interpreter::interpreter::Interpreter;
use std::io::{Write, Read};
use colored::Colorize;

struct Repl<'a> {
    interpreter: &'a mut Interpreter<'a>,
    history: Vec<String>,
}

const PROMPT: &str = "bf-interpreter> ";
const HISTORY_FILE: &str = "bf-interpreter-history.bfr";
const COMMAND_PREFIX: &str = "!";

impl Repl<'_> {
    pub fn new<'a>(interpreter: &'a mut Interpreter<'a>) -> Repl<'a> {
        Repl {
            interpreter,
            history: Vec::new(),
        }
    }

    pub fn run(mut self,
               input: &mut impl Read,
               output: &mut impl Write) -> Result<(), std::io::Error> {
        let mut code_bat = String::new();
        let mut is_loop = false;
        loop {
            output.write_all(
                if is_loop {
                    format!("{}", "... ".yellow())
                } else {
                    format!("{}", PROMPT)
                }.as_bytes()
            )?;

            let mut user_input = String::new();

            match input.read_to_string(&mut user_input) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to read input: {}", e);
                    std::process::exit(1);
                }
            }
            user_input = user_input.trim().to_string(); // Remove trailing newline

            self.history.push(user_input.clone()); // Save input to history

            if user_input.contains('[') && (!user_input.contains(']') && !is_loop) {
                let loop_start_index = user_input.find('[').unwrap();

                code_bat.push_str(&user_input[loop_start_index..]);
                is_loop = true;
                user_input = user_input[..loop_start_index].to_string();
            }

            if user_input.starts_with(COMMAND_PREFIX) {
                self.run_repl_cmd(user_input, output)?;
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
    }

    fn run_repl_cmd(&mut self, user_input: String, output: &mut impl Write) -> Result<(), std::io::Error> {
        let mut cmd = user_input.split_whitespace();
        match cmd.next() {
            Some(repl_cmd) => {
                match repl_cmd.get(COMMAND_PREFIX.len()..).unwrap_or("") {
                    "fuck" => {
                        output.write_all("Bye bye :D\n".green().as_bytes())?;
                        std::process::exit(0);
                    }
                    "array" | "a" => {
                        output.write_all(format!("Current array: {:?}\n", self.interpreter.cells).as_bytes())?;
                    }
                    "array_size" | "as" => {
                        output.write_all(format!("Current array size: {}\n",
                                                 self.interpreter.array_size.to_string().bold().green()).as_bytes())?;
                    }
                    "pointer" | "p" => {
                        output.write_all(format!("Current pointer: {}\n",
                                                 self.interpreter.pointer.to_string().bold().green()).as_bytes())?;
                    }
                    "pointer_value" | "pv" => {
                        format!(
                            "Current pointer value: {} = \'{}\' (char)\n",
                            self.interpreter.cells[self.interpreter.pointer],
                            self.interpreter.cells[self.interpreter.pointer] as char
                        );
                    }
                    "history" | "h" => {
                        output.write_all("History:\n".underline().green().as_bytes())?;
                        for (i, cmd) in self.history.iter().enumerate() {
                            output.write_all(format!("{}: {}", i, cmd).as_bytes())?;
                        }
                    }
                    "save" | "s" => {
                        let file_name = cmd.next().unwrap_or(HISTORY_FILE);

                        output.write_all(format!("Saving history to file: {file_name}").yellow().as_bytes())?;
                        match std::fs::write(file_name, self.history.join("\n")) {
                            Ok(_) => {
                                output.write_all(format!("Successfully saved history to file: {file_name}")
                                    .green().as_bytes())?;
                            }
                            Err(e) => {
                                error!("Failed to save history to file: {}", e);
                            }
                        }
                    }
                    "load" | "l" => {
                        let file_name = cmd.next().unwrap_or(HISTORY_FILE);

                        output.write_all(format!("Loading history from file: {file_name}\n").yellow().as_bytes())?;
                        match std::fs::read_to_string(file_name) {
                            Ok(history) => {
                                output.write_all(format!("Successfully loaded history from file: {file_name}")
                                    .green().as_bytes())?;
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
                        output.write_all("Resetting REPL\n".truecolor(56, 33, 102).as_bytes())?;
                        self.interpreter.reset();
                        self.history = Vec::new();
                    }
                    "help" => {
                        output.write_all("!array, !a: print the current array\n\
                        !array_size, !as: print the current array size\n\
                        !pointer, !p: print the current pointer\n\
                        !pointer_value, !pv: print the current pointer value\n\
                        !history, !h: print the REPL history\n\
                        !save, !s: save the REPL history to a file\n\
                        !load, !l: load the REPL history from a file\n\
                        !reset, !r: reset the REPL\n\
                        !help: print this help message\n\
                        !fuck: exit the REPL\n".as_bytes())?;
                    }
                    _ => output.write_all(format!("Unknown command: {}, type {} to show the help",
                                                  user_input, (COMMAND_PREFIX.to_string() + "help").green()
                    ).red().as_bytes())?,
                }
            }
            None => {}
        }
        Ok(())
    }
}

/// Run the REPL
/// # Arguments
/// * `interpreter` - The interpreter to use
pub fn start<'a>(interpreter: &'a mut Interpreter<'a>,
             input: &mut impl Read,
             output: &mut impl Write) {
    info!("Entering REPL mode");
    output.write_all(
        format!(
            "{}\n\
            Brainfuck interpreter v {}\nBy {}\n\
            {}\n\
            Type {} to exit :D\n\
            type {} to get more fu*king help\n",
            "Welcome to the brainfuck REPL mode! :)".green(),
            clap::crate_version!().to_string().yellow(),
            clap::crate_authors!().to_string().green(),
            "Enter your brainfuck code and press enter to run it.".italic().blue(),
            (COMMAND_PREFIX.to_string() + "fuck").bold().red(),
            (COMMAND_PREFIX.to_string() + "help").bold().green(),
        ).as_bytes()).unwrap_or_else(|e| error!("Failed to write to output: {}", e));


    match Repl::new(interpreter).run(input, output) {
        Ok(_) => {
            info!("Successfully ran REPL");
        }
        Err(e) => {
            error!("Failed to run REPL: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn nested_loop_level_1() {
        let mut interpreter = Interpreter::new(
            30000,
            vec![],
        );


        assert_eq!(interpreter.run(String::from("++")), Ok(0));
        assert_eq!(interpreter.run(String::from("[>++")), Ok(0));
        assert_eq!(interpreter.run(String::from("[>+<-]")), Ok(0));
        assert_eq!(interpreter.run(String::from("<-]")), Ok(0));
        assert_eq!(interpreter.cells[2], 4);
    }*/
}