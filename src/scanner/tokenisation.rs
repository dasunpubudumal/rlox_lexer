use crate::{Token, TokenType};

use super::Scanner;

impl<'a> Scanner<'a> {

    /// Check two-digit operators
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.current_char.unwrap() == expected {
            // This token is already been read
            self.code_chars.next();
            return true;   
        }
        false
    }

    /// Scans individual characters and returns a token
    pub fn scan_individual_token(&mut self, character: &char, line: usize) -> Option<Token> {
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
}