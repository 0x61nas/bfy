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
    /// If the value is you want decrement the value and the value is 0, don't set the value to 255, otherwise decrement the value.
    /// If the value is you want increment the value and the value is 255, don't set the value to 0, otherwise increment the value.
    /// The alias are: `nrv`
    #[clap(alias = "nrv")]
    NoReverseValue,
    /// If the pointer at the end of the array, set the pointer to 0, otherwise increment the pointer.
    /// If the pointer at the beginning of the array, set the pointer to the end of the array, otherwise decrement the pointer.
    /// The alias are: `rp`
    #[clap(alias = "rp")]
    ReversePointer,
    /// Allow the use of utf8 characters (32 bit), otherwise only 8 bit characters are allowed.
    /// Use this feature with caution because it increases the cell size from 8 bits to 32 bits.
    /// It also allow you to use the emoji in your brainfuck code :D,
    /// This is if you can preserve your mind so that you can access their digital value :).
    /// The `u32` in rust can only store values from 0 to 4294967295, but we can only use 0 to 1114111 (0x10FFFF) for now.
    /// The alias are: `utf8`
    #[clap(alias = "utf8")]
    AllowUtf8,
}
