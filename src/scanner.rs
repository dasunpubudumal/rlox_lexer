use std::char;

use crate::Token; 
use crate::TokenType;

/// Code is a reference. Current and previous tokens are returned and therefore not referred.
pub struct Scanner<'a> {
    pub code: &'a str,
    pub current_token: Option<Token>,
    pub previous_token: Option<Token>,
    pub current_line: usize,
}

/// We need to guarantee that the reference `code` we provide into `new()` lives throughout the Scanner instance.
/// Hence, the lifetime `'a`.
impl<'a> Scanner<'a> {
    /// Creates a new Scanner struct
    pub fn new(code: &'a str) -> Scanner<'a> {
        Scanner {
            code,
            current_token: Some(Token {
                kind: TokenType::Start,
                lexeme: String::from(""),
                literal: None,
                line: 0,
            }),
            previous_token: None,
            current_line: 0,
        }
    }

    /// Check if the source code is at an end
    fn is_at_end(&mut self) -> bool {
        self.current_line >= self.code.len() 
    }

    /// Returns an iterator that contains tokens of type `Token`.
    /// Chained token is the EOF token that makes parsing a little easier.
    pub fn scan_tokens(&'a mut self) -> impl Iterator<Item = Token> + 'a {
        let current_line = self.current_line;
        std::iter::from_fn(move || {
            // This still moves `self` into the closure.
            // Figure out a way around this? `Rc` and `RefCell` might be good candiates to solve this.
            if (self.is_at_end()) {
                Some(
                    Token {
                        kind: TokenType::Eof,
                        lexeme: String::from(""),
                        literal: None,
                        line: current_line,
                    })  
            } else {
                Some(self.next_token())
            }
        })
            .chain(std::iter::once(Token {
                kind: TokenType::Eof,
                lexeme: String::from(""),
                literal: None,
                line: current_line,
            }))
    }

    /// Scans individual characters and returns a token
    fn scan_individual_token(character: &char, line: usize) -> Option<Token> {
        match character {
            '(' => Some(Token {
                kind: TokenType::LeftParen,
                lexeme: String::from("("),
                line,
                literal: None,
            }),
            ')' => Some(Token {
                kind: TokenType::RightParen,
                lexeme: String::from(")"),
                line,
                literal: None
            }),
            '{' => Some(Token {
                kind: TokenType::LeftBrace,
                lexeme: String::from("{"),
                line,
                literal: None
            }),
            '}' => Some(Token {
                kind: TokenType::RightBrace,
                lexeme: String::from("}"),
                line,
                literal: None
            }),
            ',' => Some(Token {
                kind: TokenType::Comma,
                lexeme: String::from(","),
                line,
                literal: None
            }),
            '.' => Some(Token {
                kind: TokenType::Dot,
                lexeme: String::from("."),
                line,
                literal: None
            }),
            '-' => Some(Token {
                kind: TokenType::Minus,
                lexeme: String::from("-"),
                line,
                literal: None
            }),
            '+' => Some(Token {
                kind: TokenType::Plus,
                lexeme: String::from("+"),
                line,
                literal: None
            }),
            ';' => Some(Token {
                kind: TokenType::SemiColon,
                lexeme: String::from(";"),
                line,
                literal: None
            }),
            '*' => Some(Token {
                kind: TokenType::Star,
                lexeme: String::from("*"),
                line,
                literal: None
            }),
            _ => None
        }
    }



    /// Returns the next token. This is a private function only used by `scan_tokens()` function
    fn next_token(&mut self) -> Token {
        todo!("Yet to be implemented")
    }
}
