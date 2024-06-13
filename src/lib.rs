use std::fmt::Display;
use std::io::stdin;
use std::{fs};

use scanner::Scanner;
mod error_handler;
mod scanner;

#[derive(Debug)]
pub struct Literal {}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Different types of tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Eof,
    Start,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

/// Token type for lexical analysis
#[derive(Debug)]
pub struct Token {
    // Type of the token. I have used `kind` here as `type` is a reserved word in Rust
    pub kind: TokenType,
    // For example, in the statement int x = 10;, there would be four lexemes: `"int"`, `"x"`, `"="`, and `"10"`.
    pub lexeme: String,
    // For many tokens, this would be None, but for literals this needs to be set as its own type (e.g. `String` for strings)
    pub literal: Option<Literal>,
    // Line number of the
    pub line: usize,
}

impl Token {
    // Displays the token as a string (for debugging purposes)
    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}
/// Run the source code file
pub fn run_file(file_path: &str) -> impl Iterator<Item = Token> + '_ {
    let content = fs::read_to_string(file_path).expect("Invalid file path");
    run(&content).collect::<Vec<Token>>().into_iter()
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
    scanner.scan_tokens().collect::<Vec<Token>>().into_iter()
}
