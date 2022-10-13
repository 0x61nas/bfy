mod arguments;
mod bf_interpreter;
mod repl;
mod utils;

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

    let term = console::Term::stdout();

    info!("Initializing interpreter");
    let mut interpreter =
        Interpreter::new(args.array_size,
                         args.features.unwrap_or_else(|| vec![]),
        term);

    match args.source {
        Some(source) => {
            info!("Running brainfuck source code from file: {}", source);
            match interpreter.run(utils::read_brainfuck_code(&source)) {
                Ok(exit_code) => {
                    info!(
                        "Finished running brainfuck source code from file: {}",
                        source
                    );
                    if !args.without_tiles {
                        println!(
                            "{}",
                            format!(
                                "Successfully ran brainfuck source code from file: {}",
                                source
                            )
                            .bold()
                            .green()
                        );
                        println!(
                            "{}{}",
                            "Exiting with code: ".truecolor(33, 97, 61),
                            exit_code.to_string().bold().green()
                        );
                        std::process::exit(exit_code);
                    }
                }
                Err(e) => {
                    error!("Failed to run brainfuck source code from file: {}", e);
                    std::process::exit(e.code);
                }
            }
        }
        None => repl::start(interpreter),
    }
}
