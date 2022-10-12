mod arguments;
mod repl;
mod utils;
mod bf_interpreter;

use std::io::{Read, Write};
use clap::Parser;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use colored::Colorize;

use arguments::Args;
use bf_interpreter::interpreter::Interpreter;

fn main() {
    pretty_env_logger::init();
    info!("Initialized logger");
    info!("Parsing command line arguments");
    let args = Args::parse();
    info!("Parsed command line arguments: {:?}", args);

    let mut stdin: Box<dyn Read> = Box::new(std::io::stdin());
    let mut stdout: Box<dyn Write> = Box::new(std::io::stdout());
    info!("Initializing interpreter");
    let mut interpreter = Interpreter::new(
        args.array_size,
        &mut stdin,
        &mut stdout,
        args.features.unwrap_or_else(|| vec![]),
    );

    match args.source {
        Some(source) => {
            info!("Running brainfuck source code from file: {}", source);
            match interpreter.run(utils::read_brainfuck_code(&source)) {
                Ok(exit_code) => {
                    info!("Finished running brainfuck source code from file: {}", source);
                    if !args.without_tiles {
                        println!("{}", format!(
                            "Successfully ran brainfuck source code from file: {}",
                            source
                        ).bold().green());
                        println!("Exiting with code: {exit_code}");
                        std::process::exit(0);
                    }
                }
                Err(e) => {
                    error!("Failed to run brainfuck source code from file: {}", e);
                    std::process::exit(e.code);
                }
            }
        }
        None => repl::start(&mut interpreter, &mut std::io::stdin(), &mut std::io::stdout()),
    }
}
