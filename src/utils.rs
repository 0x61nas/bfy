pub fn read_brainfuck_code(source: &String) -> String {
    info!("Reading brainfuck source code from file: {}", source);
    match std::fs::read_to_string(source) {
        Ok(source) => clean(source).unwrap_or_else(|| {
            error!("The source code is empty");
            std::process::exit(2);
        }),
        Err(e) => {
            error!("Failed to read source code file: {}", e);
            eprintln!("Failed to read source code file: {}", e);
            std::process::exit(1);
        }
    }
}

fn clean(source: String) -> Option<String> {
    if source.is_empty() {
        return None;
    }
    let code: String = source
        .chars()
        .filter(|c| match c {
            '+' | '-' | '<' | '>' | '[' | ']' | '.' | ',' => true,
            _ => false,
        })
        .collect();
    if code.is_empty() {
        return None;
    }
    Some(code)
}
