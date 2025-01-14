use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: ./QQScript run <filename.qqs>");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    if command != "run" {
        eprintln!("Unknown command: {}. Only 'run' is supported.", command);
        return;
    }

    if !filename.ends_with(".qqs") {
        eprintln!("Error: The file must have a .qqs extension.");
        return;
    }

    let script = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read the file {}: {}", filename, err);
            return;
        }
    };

    match interpret_qqscript(&script) {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn interpret_qqscript(script: &str) -> Result<(), String> {
    let lines: Vec<&str> = script.lines().collect();

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("console.speak(") && line.ends_with(");") {
            let start = "console.speak(\"".len();
            let end = line.len() - 3;

            if end <= start {
                return Err("Invalid syntax in console.speak: missing text.".to_string());
            }

            let text = &line[start..end];

            if text.starts_with('"') && text.ends_with('"') {
                return Err("Invalid syntax in console.speak: missing quotes.".to_string());
            }

            println!("{}", text);
        } else {
            return Err(format!("Unknown command: {}", line));
        }
    }

    Ok(())
}
