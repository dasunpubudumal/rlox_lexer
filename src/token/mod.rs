use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub struct Literal<T> {
    pub(crate) kind: T,
}

impl<T> Display for Literal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    String(String),
    Int(i32),
    Float(f64),
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
pub struct Token<T: Debug> {
    // Type of the token. I have used `kind` here as `type` is a reserved word in Rust
    pub kind: TokenType,
    // For example, in the statement int x = 10;, there would be four lexemes: `"int"`, `"x"`, `"="`, and `"10"`.
    pub lexeme: String,
    // For many tokens, this would be None, but for literals this needs to be set as its own type (e.g. `String` for strings)
    pub literal: Option<Literal<T>>,
    // Line number of the
    pub line: usize,
}

#[derive(Debug)]
pub struct TokenBuilder<T> {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Literal<T>>,
    pub(crate) line: usize,
}

impl<T: Debug> TokenBuilder<T> {
    pub fn new() -> TokenBuilder<T> {
        TokenBuilder {
            kind: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line: 0,
        }
    }

    pub fn kind(mut self, kind: TokenType) -> Self {
        self.kind = kind;
        self
    }

    pub fn lexeme(mut self, lexeme: String) -> Self {
        self.lexeme = lexeme;
        self
    }

    pub fn literal(mut self, literal: Option<Literal<T>>) -> Self {
        self.literal = literal;
        self
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    pub fn build(self) -> Token<T> {
        Token {
            kind: self.kind,
            lexeme: self.lexeme,
            literal: self.literal,
            line: self.line,
        }
    }
}

impl<T: Debug> Token<T> {
    // Displays the token as a string (for debugging purposes)
    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}
