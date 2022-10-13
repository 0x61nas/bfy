use crate::arguments;
use crate::bf_interpreter::cell::Cell;
use crate::bf_interpreter::error::{InterpreterError, InterpreterErrorKind};
use std::io::Write;
use std::{char, usize, vec};

pub struct Interpreter {
    pub cells: Vec<Cell>,
    pub pointer: usize,
    pub bf_commands: Vec<BfCommand>,
    brackets: Vec<BfCommand>,
    pub features: Vec<arguments::Feature>,
    pub term: console::Term,
}

impl Interpreter {
    pub fn new(array_size: usize, features: Vec<arguments::Feature>, term: console::Term) -> Self {
        Self {
            cells: vec![Cell::default_cell(&features); array_size],
            pointer: 0,
            bf_commands: vec![],
            brackets: Vec::new(),
            features,
            term,
        }
    }

    pub fn run(&mut self, bf_code: String) -> Result<i32, InterpreterError> {
        self.bf_commands = to_bf_commands(bf_code.chars().collect())?;

        match self.run_brainfuck_code(&self.bf_commands.clone()) {
            Ok(_) => Ok(0),
            Err(e) => Err(e),
        }
    }

    // +[>++<-]
    fn iterate(&mut self, code: &Vec<BfCommand>) -> Result<(), InterpreterError> {
        trace!("Iterate: {:?}", code);
        while self.cells[self.pointer].get_value_utf8() != 0 {
            self.run_brainfuck_code(code)?;
        }
        Ok(())
    }

    fn run_brainfuck_code(&mut self, bf_code: &Vec<BfCommand>) -> Result<(), InterpreterError> {
        for command in bf_code {
            match command {
                BfCommand::IncPtr => self.increment_pointer()?,
                BfCommand::DecPtr => self.decrement_pointer()?,
                BfCommand::IncVal => self.increment_value()?,
                BfCommand::DecVal => self.decrement_value()?,
                BfCommand::Print => self.output_value()?,
                BfCommand::Read => self.input_value()?,
                BfCommand::Loop(loop_body) => self.iterate(loop_body)?,
            }
        }

        Ok(())
    }

    fn increment_pointer(&mut self) -> Result<(), InterpreterError> {
        trace!("Increment pointer");
        self.pointer += 1;
        if self.pointer >= self.cells.len() {
            if self.features.contains(&arguments::Feature::ReversePointer) {
                self.pointer = 0;
            } else {
                return Err(InterpreterErrorKind::PointerOutOfBounds(self.pointer).to_error());
            }
        }
        Ok(())
    }

    fn decrement_pointer(&mut self) -> Result<(), InterpreterError> {
        trace!("Decrement pointer");
        if self.pointer == 0 {
            if self.features.contains(&arguments::Feature::ReversePointer) {
                self.pointer = self.cells.len() - 1;
            } else {
                return Err(InterpreterErrorKind::PointerOutOfBounds(self.pointer).to_error());
            }
        } else {
            self.pointer -= 1;
        }
        Ok(())
    }

    fn increment_value(&mut self) -> Result<(), InterpreterError> {
        trace!("Increment value");
        self.cells[self.pointer]
            .increment(!self.features.contains(&arguments::Feature::NoReverseValue))?;
        Ok(())
    }

    fn decrement_value(&mut self) -> Result<(), InterpreterError> {
        trace!("Decrement value");
        self.cells[self.pointer]
            .decrement(!self.features.contains(&arguments::Feature::NoReverseValue))?;
        Ok(())
    }

    fn output_value(&mut self) -> Result<(), InterpreterError> {
        trace!("Output value");

        if self.features.contains(&arguments::Feature::AllowUtf8) {
            let c = char::from_u32(self.cells[self.pointer].get_value_utf8());
            match c {
                Some(c) => print!("{}", c),
                None => return Err(InterpreterErrorKind::InvalidUtf8.to_error()),
            }
        } else {
            print!("{}", self.cells[self.pointer].get_value() as char);
        }
        match std::io::stdout().flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(InterpreterErrorKind::FlushError(e).to_error()),
        }
    }

    fn input_value(&mut self) -> Result<(), InterpreterError> {
        trace!("Input value");
        match self.term.read_char() {
            Ok(ch) => {
                self.cells[self.pointer].set_value(ch);
                print!("{}", ch);
                match std::io::stdout().flush() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(InterpreterErrorKind::FlushError(e).to_error()),
                }
            }
            Err(e) => Err(InterpreterErrorKind::IoError(e).to_error()),
        }
    }

    pub fn reset(&mut self) {
        self.cells = vec![Cell::default_cell(&self.features); self.cells.len()];
        self.pointer = 0;
        self.brackets = Vec::new();
        self.bf_commands = Vec::new();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BfCommand {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Print,
    Read,
    Loop(Vec<BfCommand>),
}

fn to_bf_commands(bf_code: Vec<char>) -> Result<Vec<BfCommand>, InterpreterError> {
    let mut bf_commands = Vec::new();
    let mut i = 0;
    while i < bf_code.len() {
        match bf_code[i] {
            '[' => {
                let mut bracket_count = 1;
                let mut j = i + 1;
                while j < bf_code.len() {
                    match bf_code[j] {
                        '[' => bracket_count += 1,
                        ']' => bracket_count -= 1,
                        _ => (),
                    }
                    if bracket_count == 0 {
                        break;
                    }
                    j += 1;
                }
                if bracket_count != 0 {
                    return Err(InterpreterErrorKind::UnmatchedBracket.to_error());
                }
                bf_commands.push(BfCommand::Loop(to_bf_commands(bf_code[i + 1..j].to_vec())?));
                i = j;
            }
            _ => match BfCommand::from(bf_code[i]) {
                Some(command) => bf_commands.push(command),
                None => (),
            },
        }
        i += 1;
    }
    Ok(bf_commands)
}

impl BfCommand {
    fn from(c: char) -> Option<Self> {
        match c {
            '>' => Some(BfCommand::IncPtr),
            '<' => Some(BfCommand::DecPtr),
            '+' => Some(BfCommand::IncVal),
            '-' => Some(BfCommand::DecVal),
            '.' => Some(BfCommand::Print),
            ',' => Some(BfCommand::Read),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use console::Term;
    use pretty_assertions::assert_eq; // for testing only

    #[test]
    fn print_h_combine_repl() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        assert_eq!(
            interpreter.run(String::from(">+++++++++[<++++ ++++>-]<.")),
            Ok(0)
        );

        println!();
    }

    #[test]
    fn print_h_repl() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        assert_eq!(interpreter.run(String::from(">+++++++++")), Ok(0));
        assert_eq!(interpreter.run(String::from("[<++++ ++++>-]<.")), Ok(0));

        println!();
    }

    #[test]
    fn nested_loop_level_1_combine() {
        let mut interpreter = Interpreter::new(5, vec![], Term::stdout());

        assert_eq!(interpreter.run(String::from("++[>++[>+<-]<-]")), Ok(0));
        assert_eq!(interpreter.cells[2], Cell::new(4, &vec![]));

        println!();
    }

    #[test]
    fn execute_hello_world_from_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/hello_world.bf"
            ))),
            Ok(0)
        );
    }

    #[test]
    fn execute_print_hi_from_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/print_hi.bf"
            ))),
            Ok(0)
        );
    }

    #[test]
    fn execute_print_hi_yooo_from_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/print_hi_yooo.bf"
            ))),
            Ok(0)
        );
    }

    #[test]
    fn execute_print_my_first_name_from_formatted_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/print_my_first_name_formatted.bf"
            ))),
            Ok(0)
        );
    }

    #[test]
    fn execute_print_my_first_name_from_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/print_my_first_name.bf"
            ))),
            Ok(0)
        );

        assert_eq!(interpreter.cells[0], Cell::default_cell(&vec![]));
        assert_eq!(interpreter.cells[1], Cell::default_cell(&vec![]));
        assert_eq!(interpreter.cells[2], Cell::new(115, &vec![]));
        assert_eq!(interpreter.cells[3], Cell::new(96, &vec![]));
        assert_eq!(interpreter.cells[4], Cell::new(112, &vec![]));
        assert_eq!(interpreter.cells[5], Cell::new(32, &vec![]));
    }

    #[test]
    fn execute_print_my_first_name_and_last_name_from_formatted_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/print_my_first_name_and_last_name_formatted.bf"
            ))),
            Ok(0)
        );
    }

    #[test]
    fn execute_print_my_first_name_and_last_name_from_file() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        println!();

        assert_eq!(
            interpreter.run(utils::read_brainfuck_code(&String::from(
                "test_code/print_my_first_name_and_last_name.bf"
            ))),
            Ok(0)
        );
    }

    #[test]
    fn reset() {
        let mut interpreter = Interpreter::new(30000, vec![], Term::stdout());

        assert_eq!(interpreter.run(String::from(">++++")), Ok(0));

        assert_eq!(interpreter.pointer, 1);
        assert_eq!(interpreter.cells[0], Cell::new(0, &vec![]));
        assert_eq!(interpreter.cells[1], Cell::new(4, &vec![]));
        // assert_eq!(interpreter.commands, vec!['>', '+', '+', '+', '+']);

        // reset
        interpreter.reset();

        assert_eq!(interpreter.pointer, 0);
        assert_eq!(interpreter.cells[0], Cell::new(0, &vec![]));
        assert_eq!(interpreter.cells[1], Cell::new(0, &vec![]));
        assert_eq!(interpreter.bf_commands, Vec::<BfCommand>::new());
    }
}
