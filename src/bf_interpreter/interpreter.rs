use crate::{arguments, mode};
use crate::bf_interpreter::error::{InterpreterError, InterpreterErrorKind};
use std::io::{Read, Write};
use std::{char, usize, vec};

pub struct Interpreter {
    pub cells: Vec<u8>,
    pub pointer: usize,
    pub array_size: usize,
    pub bf_code: Vec<char>,
    brackets: Vec<BfCommand>,
    pub features: Vec<arguments::Feature>,
    mode: mode::RunMode,
}

impl Interpreter {
    pub fn new(
        array_size: usize,
        bf_code: Option<String>,
        features: Vec<arguments::Feature>,
        run_mode: mode::RunMode
    ) -> Self {
        trace!("Run mode{run_mode:?}");
        Self {
            cells: vec![0; array_size],
            pointer: 0,
            array_size,
            bf_code: bf_code.unwrap_or_else(|| String::new()).chars().collect(),
            brackets: Vec::new(),
            features,
            mode: run_mode,
        }
    }

    pub fn run(&mut self, bf_code: Option<String>) -> Result<i32, InterpreterError> {
        let bf_code = match bf_code {
            Some(bf_code) => {
                bf_code.chars().collect()
            }
            None => self.bf_code.clone(),
        };

        match self.run_brainfuck_code(bf_code, false) {
            Ok(_) => Ok(0),
            Err(e) => Err(e),
        }
    }

    // +[>++<-]
    fn iterate(&mut self, code: Vec<char>) -> Result<(), InterpreterError> {
        trace!("Iterate: {:?}", code);
        while self.cells[self.pointer] != 0 {
            self.run_brainfuck_code(code.clone(), true)?;
        }
        Ok(())
    }

    fn run_brainfuck_code(&mut self, bf_code: Vec<char>, from_loop: bool) -> Result<(), InterpreterError> {
        let mut removed_num = 0_usize;
        for (i, ch) in bf_code.iter().enumerate() {
            match BfCommand::from_char(ch, i - removed_num) {
                Some(cmd) => {
                    trace!("Executing command: {:?}", cmd);
                    self.execute(cmd)?;

                    // Push the char to the bf_code vector if isn't from loop and we run in REPL mode
                    if !from_loop && self.mode == mode::RunMode::Repl {
                        self.bf_code.push(ch.clone());
                    }
                }
                None => {
                    trace!("Skipping character: \'{}\'", ch);
                    removed_num += 1;
                }
            }
        }
        Ok(())
    }

    fn execute(&mut self, cmd: BfCommand) -> Result<(), InterpreterError> {
        match cmd {
            BfCommand::IncPtr => {
                self.pointer += 1;
                if self.pointer >= self.array_size {
                    if self.features.contains(&arguments::Feature::ReversePointer) {
                        self.pointer = 0;
                    } else {
                        return Err(InterpreterErrorKind::PointerOutOfBounds(self.pointer).to_error())
                    }
                }
            }
            BfCommand::DecPtr => {
                if self.pointer == 0 {
                    if self.features.contains(&arguments::Feature::ReversePointer) {
                        self.pointer = self.array_size - 1;
                    } else {
                        return Err(InterpreterErrorKind::PointerOutOfBounds(self.pointer).to_error());
                    }
                } else {
                    self.pointer -= 1;
                }
            }
            BfCommand::IncVal => {
                if self.cells[self.pointer] == 255 {
                    if self.features.contains(&arguments::Feature::ReverseValue) {
                        self.cells[self.pointer] = 0;
                    } else {
                        return Err(InterpreterErrorKind::ValueOutOfBounds.to_error());
                    }
                } else {
                    self.cells[self.pointer] += 1;
                }
            }
            BfCommand::DecVal => {
                if self.cells[self.pointer] == 0 {
                    if self.features.contains(&arguments::Feature::ReverseValue) {
                        self.cells[self.pointer] = 255;
                    } else {
                        return Err(InterpreterErrorKind::ValueOutOfBounds.to_error());
                    }
                } else {
                    self.cells[self.pointer] -= 1;
                }
            }
            BfCommand::Print => {
                print!("{}", self.cells[self.pointer] as char);
                std::io::stdout().flush().unwrap();
            }
            BfCommand::Read => {
                self.cells[self.pointer] = match std::io::stdin().bytes().next() {
                    Some(Ok(byte)) => byte,
                    Some(Err(e)) => {
                        return Err(InterpreterErrorKind::ByteReadError(e).to_error());
                    }
                    None => {
                        return Err(InterpreterErrorKind::ReadError.to_error());
                    }
                };
            }
            BfCommand::LoopStart(i) => {
                self.brackets.push(BfCommand::LoopStart(i));
            }
            BfCommand::LoopEnd(i) => {
                let open_bracket = self.brackets.pop();
                match open_bracket {
                    Some(BfCommand::LoopStart(j)) => {
                        if self.cells[self.pointer] != 0 {
                            let start = match &self.mode {
                                mode::RunMode::Repl if self.bf_code.len() - j >= i =>
                                    self.bf_code.len() - j - i + 1,
                                _ => j + 1
                            };
                            debug!("bf_code array len: {}", self.bf_code.len());
                            debug!("start index {}", start);
                            debug!("bf_code at start: {}", self.bf_code[start]);
                            debug!("i: {i}, j: {j}");
                            // debug!("{}", self.bf_code[i]);
                            let end = match &self.mode {
                                mode::RunMode::Repl => {
                                    let mut s = i + start - 2;

                                    if s >= self.bf_code.len() {
                                        s = s - (self.bf_code.len() - start) + 1;
                                    }

                                    s
                                },
                                mode::RunMode::Execute => i - 1,
                            };
                            let range = start..=end;
                            debug!("{range:?}");
                            let code = self.bf_code[range].to_vec();
                            self.iterate(code)?;
                        }
                    }
                    _ => {
                        return Err(InterpreterErrorKind::UnmatchedClosingBracket(i).to_error());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.cells = vec![0; self.array_size];
        self.pointer = 0;
        self.brackets = Vec::new();
        self.bf_code = Vec::new();
    }
}

#[derive(Debug, PartialEq)]
enum BfCommand {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Print,
    Read,
    LoopStart(usize),
    LoopEnd(usize),
}

impl BfCommand {
    fn from_char(c: &char, index: usize) -> Option<BfCommand> {
        match c {
            '>' => Some(BfCommand::IncPtr),
            '<' => Some(BfCommand::DecPtr),
            '+' => Some(BfCommand::IncVal),
            '-' => Some(BfCommand::DecVal),
            '.' => Some(BfCommand::Print),
            ',' => Some(BfCommand::Read),
            '[' => Some(BfCommand::LoopStart(index)),
            ']' => Some(BfCommand::LoopEnd(index)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::RunMode;
    use pretty_assertions::assert_eq;
    use crate::utils; // for testing only

    #[test]
    fn print_h_combine_repl() {
        let mut interpreter = Interpreter::new(
            30000,
            None,
            vec![],
            RunMode::Repl
        );

        assert_eq!(interpreter.run(None), Ok(0));

        assert_eq!(interpreter.run(Some(String::from(">+++++++++[<++++ ++++>-]<."))), Ok(0));
    }
    #[test]
    fn print_h_repl() {
        let mut interpreter = Interpreter::new(
            30000,
            None,
            vec![],
            RunMode::Repl
        );

        assert_eq!(interpreter.run(None), Ok(0));

        assert_eq!(interpreter.run(Some(String::from(">+++++++++"))), Ok(0));
        assert_eq!(interpreter.run(Some(String::from("[<++++ ++++>-]<."))), Ok(0));
    }

    #[test]
    fn execute_hello_world_from_file() {
        let mut interpreter = Interpreter::new(
            30000,
            utils::read_brainfuck_code_if_any(&Some(String::from("test_code/hello_world.bf"))),
            vec![],
            RunMode::Execute
        );

        assert_eq!(interpreter.run(None), Ok(0));
    }

    #[test]
    fn execute_print_hi_from_file() {
        let mut interpreter = Interpreter::new(
            30000,
            utils::read_brainfuck_code_if_any(&Some(String::from("test_code/print_hi.bf"))),
            vec![],
            RunMode::Execute
        );

        assert_eq!(interpreter.run(None), Ok(0));
    }

    #[test]
    fn execute_print_hi_yooo_from_file() {
        let mut interpreter = Interpreter::new(
            30000,
            utils::read_brainfuck_code_if_any(&Some(String::from("test_code/print_hi_yooo.bf"))),
            vec![],
            RunMode::Execute
        );

        assert_eq!(interpreter.run(None), Ok(0));
    }

    #[test]
    fn reset() {
        let mut interpreter = Interpreter::new(
            30000,
            None,
            vec![],
            RunMode::Repl
        );

        assert_eq!(interpreter.run(None), Ok(0));

        assert_eq!(interpreter.run(Some(String::from(">++++"))), Ok(0));

        assert_eq!(interpreter.pointer, 1);
        assert_eq!(interpreter.cells[0], 0);
        assert_eq!(interpreter.cells[1], 4);
        assert_eq!(interpreter.bf_code, vec!['>', '+', '+', '+' , '+']);

        // reset
        interpreter.reset();

        assert_eq!(interpreter.pointer, 0);
        assert_eq!(interpreter.cells[0], 0);
        assert_eq!(interpreter.cells[1], 0);
        assert_eq!(interpreter.bf_code, Vec::<char>::new());

        assert_eq!(interpreter.run(None), Ok(0));
    }

    #[test]
    fn test_from_char() {
        assert_eq!(BfCommand::from_char(&'>', 0), Some(BfCommand::IncPtr));
        assert_eq!(BfCommand::from_char(&'<', 0), Some(BfCommand::DecPtr));
        assert_eq!(BfCommand::from_char(&'+', 0), Some(BfCommand::IncVal));
        assert_eq!(BfCommand::from_char(&'-', 0), Some(BfCommand::DecVal));
        assert_eq!(BfCommand::from_char(&'.', 0), Some(BfCommand::Print));
        assert_eq!(BfCommand::from_char(&',', 0), Some(BfCommand::Read));
        assert_eq!(BfCommand::from_char(&'[', 0), Some(BfCommand::LoopStart(0)));
        assert_eq!(BfCommand::from_char(&']', 0), Some(BfCommand::LoopEnd(0)));
        assert_eq!(BfCommand::from_char(&' ', 0), None);
    }
}
