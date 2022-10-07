use std::io::{Read, Write};
use std::usize;
use crate::arguments;

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

    pub fn run(&mut self, bf_code: Option<String>) -> Result<i32, (String, i32)> {
        let bf_code = match bf_code {
            Some(bf_code) => {
                self.bf_code.push_str(&*bf_code);
                bf_code
            }
            None => self.bf_code.clone()
        };

        match self.run_brainfuck_code(&bf_code) {
            Ok(_) => Ok(0),
            Err(e) => Err((e, 1)),
        }
    }

    // +[>++<-]
    fn iterate(&mut self, code: String) -> Result<(), String> {
        while self.cells[self.pointer] != 0 {
            self.run_brainfuck_code(&code)?;
        }
        Ok(())
    }


    fn run_brainfuck_code(&mut self, bf_code: &str) -> Result<(), String> {
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

    fn execute(&mut self, cmd: BfCommand) -> Result<(), String> {
        match cmd {
            BfCommand::IncPtr => {
                self.pointer += 1;
                if self.pointer >= self.array_size {
                    if self.features.contains(&arguments::Feature::ReversePointer) {
                        self.pointer = 0;
                    } else {
                        return Err(format!("Pointer out of bounds: {}", self.pointer));
                    }
                }
            }
            BfCommand::DecPtr => {
                if self.pointer == 0 {
                    if self.features.contains(&arguments::Feature::ReversePointer) {
                        self.pointer = self.array_size - 1;
                    } else {
                        return Err(format!("Pointer out of bounds: {}", self.pointer));
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
                        return Err(format!("Failed to read byte from stdin: {}", e));
                    }
                    None => {
                        return Err("Failed to read byte from stdin: EOF".to_string());
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
                        return Err(format!("Unmatched closing bracket at position: {}", i));
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
