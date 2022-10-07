fn read_brainfuck_code_if_any(source: Option<String>) -> Option<String> {
    match source {
        Some(source) => {
            info!("Reading brainfuck source code from file: {}", source);
            match std::fs::read_to_string(source) {
                Ok(source) => Some(source),
                Err(e) => {
                    error!("Failed to read source code file: {}", e);
                    eprintln!("Failed to read source code file: {}", e);
                    std::process::exit(1);
                }
            }
        },
        None => {
            None
        }
    }
}