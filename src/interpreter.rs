use std::io::{Read, Write};
use std::usize;
use crate::arguments;
use crate::interpreter::error::InterpreterError;

pub struct Interpreter {
    pub cells: Vec<u8>,
    pub pointer: usize,
    pub array_size: usize,
    pub bf_code: String,
    brackets: Vec<BfCommand>,
    pub features: Vec<arguments::Feature>,
}

impl Interpreter {
    pub fn new(array_size: usize,
               bf_code: Option<String>,
               features: Vec<arguments::Feature>) -> Self {
        Self {
            cells: vec![0; array_size],
            pointer: 0,
            array_size,
            bf_code: bf_code.unwrap_or_else(|| String::new()),
            brackets: Vec::new(),
            features,
        }
    }

    pub fn run(&mut self, bf_code: Option<String>) -> Result<i32, InterpreterError> {
        let bf_code = match bf_code {
            Some(bf_code) => {
                self.bf_code.push_str(&*bf_code);
                bf_code
            }
            None => self.bf_code.clone()
        };

        match self.run_brainfuck_code(&bf_code) {
            Ok(_) => Ok(0),
            Err(e) => Err(e)
        }
    }

    // +[>++<-]
    fn iterate(&mut self, code: String) -> Result<(), InterpreterError> {
        while self.cells[self.pointer] != 0 {
            self.run_brainfuck_code(&code)?;
        }
        Ok(())
    }


    fn run_brainfuck_code(&mut self, bf_code: &str) -> Result<(), error::InterpreterError> {
        for (i, ch) in bf_code.chars().enumerate() {
            match BfCommand::from_char(ch, i) {
                Some(cmd) => {
                    trace!("Executing command: {:?}", cmd);
                    self.execute(cmd)?
                }
                None => {
                    trace!("Skipping character: {}", ch);
                }
            }
        }
        Ok(())
    }

    fn execute(&mut self, cmd: BfCommand) -> Result<(), error::InterpreterError> {
        match cmd {
            BfCommand::IncPtr => {
                self.pointer += 1;
                if self.pointer >= self.array_size {
                    if self.features.contains(&arguments::Feature::ReversePointer) {
                        self.pointer = 0;
                    } else {
                        return Err(error::InterpreterError::new(
                            format!("Pointer out of bounds {}", self.pointer),
                            11,
                        ));
                    }
                }
            }
            BfCommand::DecPtr => {
                if self.pointer == 0 {
                    if self.features.contains(&arguments::Feature::ReversePointer) {
                        self.pointer = self.array_size - 1;
                    } else {
                        return Err(error::InterpreterError::new(
                            format!("Pointer out of bounds {}", self.pointer),
                            11,
                        ));
                    }
                } else {
                    self.pointer -= 1;
                }
            },
            BfCommand::IncVal => {
                self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(1);
            },
            BfCommand::DecVal => {
                self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(1);
            },
            BfCommand::Print => {
                print!("{}", self.cells[self.pointer] as char);
                std::io::stdout().flush().unwrap();
            },
            BfCommand::Read => {
                self.cells[self.pointer] = match std::io::stdin().bytes().next() {
                    Some(Ok(byte)) => byte,
                    Some(Err(e)) => {
                        return Err(error::InterpreterError::new(
                            format!("Failed to read byte from stdin: {}", e),
                            12,
                        ));
                    }
                    None => {
                        return Err(InterpreterError::new(
                            "Failed to read byte from stdin: no bytes available".to_string(),
                            13,
                        ));
                    }
                };
            },
            BfCommand::LoopStart(i) => {
                self.brackets.push(BfCommand::LoopStart(i));
            },
            BfCommand::LoopEnd(i) => {
                let open_bracket = self.brackets.pop();
                match open_bracket {
                    Some(BfCommand::LoopStart(j)) => {
                        if self.cells[self.pointer] != 0 {
                            let code = self.bf_code[j..i].to_string();
                            self.iterate(code)?;
                        }
                    },
                    _ => {
                        return Err(error::InterpreterError::new(
                            format!("Unmatched closing bracket at position {}", i),
                            14,
                        ));
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
    fn from_char(c: char, index: usize) -> Option<BfCommand> {
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

pub mod error {
    use std::fmt::{Debug, Formatter};

    pub struct InterpreterError {
        message: String,
        pub(crate) code: i32,
    }

    impl InterpreterError {
        pub fn new(message: String, code: i32) -> Self {
            Self {
                message: message.to_string(),
                code,
            }
        }
    }

    impl std::fmt::Display for InterpreterError {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Debug for InterpreterError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, code: {}", self.message, self.code)
        }
    }

    impl std::error::Error for InterpreterError {
        fn description(&self) -> &str {
            &self.message
        }
    }
}
