use crate::bf_interpreter::interpreter::Interpreter;
use crate::repl::repl::{Repl, COMMAND_PREFIX};
use colored::Colorize;

/// Run the REPL
/// # Arguments
/// * `interpreter` - The interpreter to use
pub fn start(interpreter: Interpreter) {
    info!("Entering REPL mode");
    println!(
        "{}\n\
            Brainfuck interpreter v {}\nBy {}\n\
            {}\n\
            Type {} to exit :D\n\
            type {} to get more fu*king help",
        "Welcome to the brainfuck REPL mode! :)".green(),
        clap::crate_version!().to_string().yellow(),
        clap::crate_authors!().to_string().green(),
        "Enter your brainfuck code and press enter to run it."
            .italic()
            .blue(),
        (COMMAND_PREFIX.to_string() + "fuck").bold().red(),
        (COMMAND_PREFIX.to_string() + "help").bold().green(),
    );

    match Repl::new(interpreter).run() {
        Ok(_) => {
            info!("Successfully ran REPL");
        }
        Err(e) => {
            error!("Failed to run REPL: {}", e);
            std::process::exit(1);
        }
    }
}
