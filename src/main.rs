mod ast;
mod helpers;
mod lexer;
mod parser;
mod token;

use std::{env, fs, process::exit};

use crate::{lexer::Lexer, parser::Parser};

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

            if lexer.encountered_error {
                exit(65);
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut lexer = Lexer::new(file_contents);
            let tokens = lexer.analyze();

            if lexer.encountered_error {
                exit(65);
            }

            let mut parser = Parser::new(tokens);
            let expr = parser.parse();
            println!("{}", match expr {
                Some(ex) => ex.to_string(),
                None => String::new(),
            });

            if parser.encountered_error {
                exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
