mod arguments;
mod interpreter;
mod utils;

use std::io::Write;
use clap::Parser;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use arguments::Args;
use crate::utils::read_brainfuck_code_if_any;


fn main() {
    pretty_env_logger::init();
    info!("Initialized logger");
    info!("Parsing command line arguments");
    let args = Args::parse();
    info!("Parsed command line arguments: {:?}", args);

    if args.verbose {
        info!("Verbose mode enabled");
    }

    info!("Initializing interpreter");
    let mut interpreter = interpreter::Interpreter::new(
        args.array_size,
        read_brainfuck_code_if_any(&args.source),
        args.features.unwrap_or_else(|| vec![]));

    match args.source {
        Some(source) => {
            info!("Running brainfuck source code from file: {}", source);
            interpreter.run(None).unwrap();
        },
        None => {
            info!("Entering REPL mode");
            println!("Welcome to the brainfuck REPL mode! :)");
            println!("Brainfuck interpreter v {}\nBy {}",
                     clap::crate_version!(), clap::crate_authors!());
            println!("Enter your brainfuck code and press enter to run it.");
            println!("Enter !fuck to exit :D");
            println!("Enter !help fuck to get more help");

            loop {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.starts_with("!") {
                    match input.trim().get(1..).unwrap() {
                        "fuck" => {
                            println!("Bye bye :D");
                            break;
                        },
                        "array" | "a" => {
                            println!("Current array: {:#?}", interpreter.cells);
                        },
                        "array_size" | "as" => {
                            println!("Current array size: {}", interpreter.array_size);
                        },
                        "pointer" | "p" => {
                            println!("Current pointer: {}", interpreter.pointer);
                        },

                        "help" => {
                            println!("!fuck: exit the REPL mode");
                            println!("!array, !a: print the current array");
                            println!("!array_size, !as: print the current array size");
                            println!("!pointer, !p: print the current pointer value");
                            println!("!history: print the history of the commands");
                            println!("!help: show this fu*king help message");
                        },
                        _ => println!("Unknown command: {}, type !help to show the help", input.trim())
                    }
                } else {
                    interpreter.run(Some(input));
                }
            }
        }
    }
}
