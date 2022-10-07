mod arguments;
mod interpreter;

use clap::Parser;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use arguments::Args;

fn main() {
    pretty_env_logger::init();
    info!("Initialized logger");
    info!("Parsing command line arguments");
    let args = Args::parse();
    info!("Parsed command line arguments: {:?}", args);

    if args.verbose {
        info!("Verbose mode enabled");
    }

    match args.source {
        Some(source) => {
            info!("Running brainfuck source code from file: {}", source);
            interpreter::run(
                match std::fs::read_to_string(source) {
                    Ok(source) => source,
                    Err(e) => {
                        error!("Failed to read source code file: {}", e);
                        eprintln!("Failed to read source code file: {}", e);
                        std::process::exit(1);
                    }
                },
                args.array_size
            )
        },
        None => {
            info!("Entering REPL mode");
        }
    }
}
