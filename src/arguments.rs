use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, version)]
pub struct Args {
    /// To be verbose
    #[arg(short, long)]
    pub verbose: bool,
    /// The brainfuck source code file to run (if not will be entered in REPL mode)
    #[arg(short, long, default_value = None)]
    pub source: Option<String>,
    /// The brainfuck array size
    #[arg(short, long, default_value = "30000")]
    pub array_size: usize,
}