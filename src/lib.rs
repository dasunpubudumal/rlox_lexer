use std::fs;
use std::io::stdin;

use scanner::Scanner;
mod scanner;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    EOF
}

pub struct Token {
    kind: TokenKind
}

/// Run the source code file
pub fn run_file(file_path: &str) -> impl Iterator<Item = Token> + '_ {
    let content = fs::read_to_string(file_path).expect("Invalid file path");
    run(&content).collect::<Vec<_>>().into_iter()
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
fn run(line: &str) -> impl Iterator<Item = Token> + '_ {
    let mut scanner = Scanner::new(line);
    std::iter::from_fn(move || {
        let next_token = scanner.next_token();
        if next_token.kind == TokenKind::EOF {
            Some(next_token)
        } else {
            None
        }
    })
}