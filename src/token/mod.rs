use std::fmt::Display;

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

#[derive(Debug)]
pub struct TokenBuilder {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Literal>,
    pub(crate) line: usize,
}

impl TokenBuilder {
    pub fn new() -> TokenBuilder {
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

    pub fn literal(mut self, literal: Option<Literal>) -> Self {
        self.literal = literal;
        self
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    pub fn build(self) -> Token {
        Token {
            kind: self.kind,
            lexeme: self.lexeme,
            literal: self.literal,
            line: self.line,
        }
    }
}

impl Token {
    // Displays the token as a string (for debugging purposes)
    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}
