mod lexer;
mod token;

use std::env;
use std::fs;

use crate::lexer::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut lexer = Lexer::new(file_contents);
            let tokens = lexer.analyze();

            for tok in tokens {
                println!("{}", tok)
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
