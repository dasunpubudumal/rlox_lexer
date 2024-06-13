use crate::token::{Token, TokenBuilder, TokenType};

use super::Scanner;

impl<'a> Scanner<'a> {
    /// Check two-digit operators
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.code_chars.peek().map(|&c| c) {
            Some(character) => {
                if character == expected {
                    self.code_chars.next();
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// Scans individual characters and returns a token
    pub fn scan_individual_token(&mut self, character: &char, line: usize) -> Option<Token> {
        match character {
            '(' => Some(
                TokenBuilder::new()
                    .kind(TokenType::LeftParen)
                    .lexeme(r"(")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            ')' => Some(
                TokenBuilder::new()
                    .kind(TokenType::RightParen)
                    .lexeme(r")")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '{' => Some(
                TokenBuilder::new()
                    .kind(TokenType::LeftBrace)
                    .lexeme(r"{")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '}' => Some(
                TokenBuilder::new()
                    .kind(TokenType::RightBrace)
                    .lexeme(r"}")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            ',' => Some(
                TokenBuilder::new()
                    .kind(TokenType::Comma)
                    .lexeme(r",")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '.' => Some(
                TokenBuilder::new()
                    .kind(TokenType::Dot)
                    .lexeme(r".")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '-' => Some(
                TokenBuilder::new()
                    .kind(TokenType::Minus)
                    .lexeme(r"-")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '+' => Some(
                TokenBuilder::new()
                    .kind(TokenType::Plus)
                    .lexeme(r"+")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            ';' => Some(
                TokenBuilder::new()
                    .kind(TokenType::SemiColon)
                    .lexeme(r";")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '*' => Some(
                TokenBuilder::new()
                    .kind(TokenType::Star)
                    .lexeme(r"*")
                    .line(line)
                    .literal(None)
                    .build(),
            ),
            '!' => {
                if self.match_char('=') {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::BangEqual)
                            .lexeme(r"!=")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                } else {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::Bang)
                            .lexeme(r"!")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                }
            }
            '=' => {
                if self.match_char('=') {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::EqualEqual)
                            .lexeme(r"==")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                } else {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::Equal)
                            .lexeme(r"=")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                }
            }
            '<' => {
                if self.match_char('=') {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::LessEqual)
                            .lexeme(r"<=")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                } else {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::Less)
                            .lexeme(r"<")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                }
            }
            '>' => {
                if self.match_char('=') {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::GreaterEqual)
                            .lexeme(r">=")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                } else {
                    Some(
                        TokenBuilder::new()
                            .kind(TokenType::Greater)
                            .lexeme(r">")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                }
            }
            _ => None,
        }
    }
}
