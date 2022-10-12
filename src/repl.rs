use crate::bf_interpreter::interpreter::Interpreter;
use colored::Colorize;
use std::io::Write;
use console::{Term, Key};

struct Repl {
    pub interpreter: Interpreter,
    term: console::Term,
    history: Vec<String>,
    loop_body: String,
    loop_depth: usize,
}

const PROMPT: &str = "bf-interpreter> ";
const HISTORY_FILE: &str = "bf-interpreter-history.bfr";
const COMMAND_PREFIX: &str = "!";

impl Repl {
    pub fn new(interpreter: Interpreter, term: Term) -> Repl {
        Repl {
            interpreter,
            term,
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
                        let last = self.history.get(self.history.len() - 1 - rev_index)
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
                        let first = self.history.get(self.history.len() - rev_index)
                            .unwrap();
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
        match user_input.find('[') {
            Some(index) if self.loop_depth == 0 => {
                self.loop_body.push_str(&user_input[index..]);
                self.loop_depth = 1;
                user_input = user_input[..index].to_string();
            }
            Some(_) => {
                self.loop_body.push_str(&user_input);
                user_input.matches('[').for_each(|_| self.loop_depth += 1);
                user_input.matches(']').for_each(|_| self.loop_depth -= 1);
                return;
            }
            _ => {
                if user_input.contains(']') {
                    if self.loop_depth == 0 {
                        error!("Found ']' without matching '['");
                        return;
                    }
                    self.loop_depth -= 1;
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
                            self.interpreter.cells[self.interpreter.pointer].to_char().unwrap_or_else(|_| '?')
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

/// Run the REPL
/// # Arguments
/// * `interpreter` - The interpreter to use
pub fn start(interpreter: Interpreter, term: Term) {
    info!("Entering REPL mode");
    println!(
        "{}\n\
            Brainfuck interpreter v {}\nBy {}\n\
            {}\n\
            Type {} to exit :D\n\
            type {} to get more fu*king help",
        "Welcome to the brainfuck REPL mode! :)".green(),
        clap::crate_version!().to_string().yellow(),
        clap::crate_authors!().to_string().green(),
        "Enter your brainfuck code and press enter to run it."
            .italic()
            .blue(),
        (COMMAND_PREFIX.to_string() + "fuck").bold().red(),
        (COMMAND_PREFIX.to_string() + "help").bold().green(),
    );

    match Repl::new(interpreter, term).run() {
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
    use pretty_assertions::assert_eq;
    use crate::bf_interpreter::cell::Cell;

    #[test]
    fn nested_loop_level_1() {
        let mut term = Term::stdout();
        let interpreter = Interpreter::new(4, vec![], &mut term);

        let mut repl = Repl::new(interpreter, term);

        repl.process("++".to_string());
        repl.process("[>++".to_string());
        repl.process("[>+<-]".to_string());
        repl.process("<-]".to_string());

        let cells = &repl.interpreter.cells;

        assert_eq!(cells[0], Cell::default_cell(&vec![]));
        assert_eq!(cells[1], Cell::default_cell(&vec![]));
        assert_eq!(cells[2], Cell::new(4, &vec![]));
    }

    #[test]
    fn nested_loop_level_2() {
        let mut term = console::Term::stdout();
        let interpreter = Interpreter::new(4, vec![], &mut term);

        let mut repl = Repl::new(interpreter, term);

        repl.process("++".to_string());
        repl.process("[>++".to_string());
        repl.process("[>+<-]".to_string());
        repl.process("[>++".to_string());
        repl.process("[>+<-]".to_string());
        repl.process("<-]".to_string());
        repl.process("<-]".to_string());

        let cells = &repl.interpreter.cells;

        assert_eq!(cells[0], Cell::default_cell(&vec![]));
        assert_eq!(cells[1], Cell::default_cell(&vec![]));
        assert_eq!(cells[2], Cell::new(4, &vec![]));
    }

    #[test]
    fn print_my_first_name() {
        let mut term = console::Term::stdout();
        let interpreter = Interpreter::new(10, vec![], &mut term);

        let mut repl = Repl::new(interpreter, term);

        let code = "++++ ++++ 8
        [
            >++++
            [
            >++ A
            >+++ a
            >++++
            >+ space
            <<<<-
        ]

        >>>>>>++
        [
            <<<-
            >>>-
        ]

        <<<<<<<-
        ]
        >>+. Print cell 2: A
        <<++++
            [
            >+++
            [
            >+++
            <-
        ]
        >++
        <<-
        ]
        >>+. Print n
            <<+++
            [
            >+++
            [
            >-
            <-
        ]
        >-
            <<-
        ]
        >>-. Print n
            <<++++++
            [
            >>+++
            <<-
        ]
        >>. Print s"
            .to_string()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for line in code {
            repl.process(line);
        }
    }

    #[test]
    fn print_my_first_name_in_one_command() {
        let mut term = console::Term::stdout();
        let interpreter = Interpreter::new(10, vec![], &mut term);

        let mut repl = Repl::new(interpreter, term);

        let code = "++++++++[>++++[>++<-]>>>>>>++[<<<->>>-]<<<<<<<-]>>+.<<++++[>+++
        [>+++<-]>++<<-]>>+.<<+++[>+++[>-<-]>-<<-]>>-.<<++++++[>>+++<<-]>>."
            .to_string();

        repl.process(code);
    }

    #[test]
    fn print_hello_world() {
        let mut term = console::Term::stdout();
        let interpreter = Interpreter::new(10, vec![], &mut term);

        let mut repl = Repl::new(interpreter, term);

        let _ = "[ This program prints \"Hello World!\" and a newline to the screen, its
                length is 106 active command characters. [It is not the shortest.]
                ]
                ++++++++               Set Cell #0 to 8
                [
                    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
                    [                   as the cell will be cleared by the loop
                        >++             Add 2 to Cell #2
                        >+++            Add 3 to Cell #3
                        >+++            Add 3 to Cell #4
                        >+              Add 1 to Cell #5
                        <<<<-           Decrement the loop counter in Cell #1
                    ]                   Loop until Cell #1 is zero; number of iterations is 4
                    >+                  Add 1 to Cell #2
                    >+                  Add 1 to Cell #3
                    >-                  Subtract 1 from Cell #4
                    >>+                 Add 1 to Cell #6
                    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
                    <-                  Decrement the loop Counter in Cell #0
                ]                       Loop until Cell #0 is zero; number of iterations is 8

                The result of this is:
                Cell no :   0   1   2   3   4   5   6
                Contents:   0   0  72 104  88  32   8
                Pointer :   ^

                >>.                     Cell #2 has value 72 which is 'H'
                >---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
                +++++++..+++.           Likewise for 'llo' from Cell #3
                >>.                     Cell #5 is 32 for the space
                <-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
                <.                      Cell #3 was set to 'o' from the end of 'Hello'
                +++.------.--------.    Cell #3 for 'rl' and 'd'
                >>+.                    Add 1 to Cell #5 gives us an exclamation point
                >++.                    And finally a newline from Cell #6
            "
            .to_string()
            .split("\n")
            .for_each(|s| repl.process(s.to_string()));
    }
}
