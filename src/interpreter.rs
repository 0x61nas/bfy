use std::io::{Read, Write};
use std::usize;
use crate::arguments;

pub struct Interpreter {
    pub array: Vec<u8>,
    pub pointer: usize,
    pub array_size: usize,
    pub bf_code: String,
    pub brackets: Vec<BfCommand>,
    pub features: Vec<arguments::Feature>,
}
impl Interpreter {
    pub fn new(array_size: usize,
               bf_code: Option<String>,
               features: Vec<arguments::Feature>) -> Self {
        Self {
            array: vec![0; array_size],
            pointer: 0,
            array_size,
            bf_code: bf_code.unwrap_or_else(|| String::new()),
            brackets: Vec::new(),
            features,
        }
    }

    pub fn run(&mut self, bf_code: Option<String>) {
        let mut cells = vec![0u8; bf_arr_size];
        let mut ptr = 0;
        let mut brackets = vec![];

        for (i, ch) in bf_code.chars().enumerate() {
            trace!("Current character: {}", ch);
            trace!("Current pointer: {}", ptr);
            trace!("Current cell: {}", cells[ptr]);

            match BfCommand::from_char(ch, i) {
                Some(cmd) => {
                    trace!("Executing command: {:?}", cmd);
                    match cmd {
                        BfCommand::IncPtr => {
                            if ptr == bf_arr_size - 1 {
                                eprintln!("Error: pointer out of bounds");
                            } else {
                                ptr += 1;
                            }
                        },
                        BfCommand::DecPtr => ptr -= 1,
                        BfCommand::IncVal => {
                            cells[ptr] += 1
                        },
                        BfCommand::DecVal => cells[ptr] -= 1,
                        BfCommand::Print => {
                            trace!("Printing value: {}", cells[ptr]);
                            println!("{}", cells[ptr]);
                            std::io::stdout().flush().unwrap();
                        },
                        BfCommand::Read => cells[ptr] = std::io::stdin()
                            .bytes().next().unwrap().unwrap() as u8,
                        BfCommand::LoopStart(index) => brackets.push(cmd),
                        BfCommand::LoopEnd => {
                            if cells[ptr] != 0 {
                                let i = brackets
                                    .iter()
                                    .map(|cmd| match cmd {
                                        BfCommand::LoopStart(index) => *index,
                                        _ => unreachable!()
                                    })
                                    .last();
                                brackets.truncate(i);
                                ptr = index;
                            } else {
                                brackets.pop();
                            }
                        }
                    }
                },
                None => {
                    trace!("Ignoring character: {}", ch);
                } // Ignore unknown characters
            }
        }
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
    LoopEnd,
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
            ']' => Some(BfCommand::LoopEnd),
            _ => None,
        }
    }
}