pub fn read_brainfuck_code(source: &String) -> String {
    info!("Reading brainfuck source code from file: {}", source);
    match std::fs::read_to_string(source) {
        Ok(source) => clean(source),
        Err(e) => {
            error!("Failed to read source code file: {}", e);
            eprintln!("Failed to read source code file: {}", e);
            std::process::exit(1);
        }
    }
}

fn clean(source: String) -> String {
    source
        .chars()
        .filter(|c| match c {
            '+' | '-' | '<' | '>' | '[' | ']' | '.' | ',' => true,
            _ => false,
        })
        .collect()
}
