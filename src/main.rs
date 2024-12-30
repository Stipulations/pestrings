use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn extract_strings(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let strings: Vec<String> = buffer
        .split(|&byte| !byte.is_ascii_graphic() && byte != b' ')
        .filter(|chunk| chunk.len() > 3)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    Ok(strings)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: pestrings <path/to/executable>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    match extract_strings(file_path) {
        Ok(strings) => {
            for string in strings {
                println!("{}", string);
            }
            println!("\nPress Enter to close...");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    }
}
