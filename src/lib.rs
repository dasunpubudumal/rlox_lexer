use std::fs;
use std::io::stdin;

use scanner::Scanner;
use token::{LiteralType, Token};
mod constants;
mod error_handler;
mod scanner;
mod token;

/// Run the source code file
pub fn run_file(file_path: &str) -> impl Iterator<Item = Token<LiteralType>> + '_ {
    let content = fs::read_to_string(file_path).expect("Invalid file path");
    run(&content)
        .collect::<Vec<Token<LiteralType>>>()
        .into_iter()
}

/// Run REPL input
pub fn run_prompt() {
    loop {
        let mut line = String::new();
        stdin()
            .read_line(&mut line)
            .expect("Failed to read the line.");
        run(&line);
    }
}

/// Run either the source code or REPL line
fn run(line: &str) -> impl Iterator<Item = Token<LiteralType>> + '_ {
    let scanner = Scanner::new(line);
    scanner.scan_tokens().tokens.into_iter()
}
