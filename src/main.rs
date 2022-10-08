mod arguments;
mod repl;
mod utils;
mod bf_interpreter;

use clap::Parser;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use arguments::Args;
use bf_interpreter::interpreter::Interpreter;

fn main() {
    pretty_env_logger::init();
    info!("Initialized logger");
    info!("Parsing command line arguments");
    let args = Args::parse();
    info!("Parsed command line arguments: {:?}", args);

    info!("Initializing bf_interpreter");
    let mut interpreter = Interpreter::new(
        args.array_size,
        utils::read_brainfuck_code_if_any(&args.source),
        args.features.unwrap_or_else(|| vec![]),
    );

    match args.source {
        Some(source) => {
            info!("Running brainfuck source code from file: {}", source);
            match interpreter.run(None) {
                Ok(exit_code) => {
                    info!("Finished running brainfuck source code from file: {}", source);
                    if !args.without_tiles {
                        println!(
                            "Successfully ran brainfuck source code from file: {}",
                            source
                        );
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
        None => repl::start(interpreter),
    }
}
