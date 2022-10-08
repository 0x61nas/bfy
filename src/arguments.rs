use clap::{arg, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, version)]
pub struct Args {
    /// The brainfuck source code file to run (if not will be entered in REPL mode)
    #[arg(default_value = None)]
    pub source: Option<String>,
    #[arg(short, long, default_value = None)]
    pub features: Option<Vec<Feature>>,
    /// The brainfuck array size
    #[arg(short, long, default_value = "30000")]
    pub array_size: usize,
    /// Dont print the tiles (e.g. exit code, file name, etc)
    #[arg(short, long)]
    pub without_tiles: bool,
}

#[derive(Debug, PartialEq, Copy, Clone, ValueEnum)]
pub enum Feature {
    /// If the value is you want decrement the value and the value is 0, set the value to 255, otherwise decrement the value.
    /// If the value is you want increment the value and the value is 255, set the value to 0, otherwise increment the value.
    ReverseCounter,
    /// If the pointer at the end of the array, set the pointer to 0, otherwise increment the pointer.
    /// If the pointer at the beginning of the array, set the pointer to the end of the array, otherwise decrement the pointer.
    ReversePointer,
}
