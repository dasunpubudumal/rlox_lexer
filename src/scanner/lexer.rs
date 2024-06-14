use std::string::ParseError;

use crate::token::{TokenBuilder, TokenType};

use super::Scanner;

use crate::constants::NEWLINE;
use crate::error_handler::ParserError;

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
    pub fn scan_individual_token(
        &mut self,
        character: &char,
        line: usize,
    ) -> Result<(), ParserError> {
        match character {
            '(' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::LeftParen)
                        .lexeme(r"(")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            ')' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::RightParen)
                        .lexeme(r")")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '{' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::LeftBrace)
                        .lexeme(r"{")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '}' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::RightBrace)
                        .lexeme(r"}")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            ',' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::Comma)
                        .lexeme(r",")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '.' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::Dot)
                        .lexeme(r".")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '-' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::Minus)
                        .lexeme(r"-")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '+' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::Plus)
                        .lexeme(r"+")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            ';' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::SemiColon)
                        .lexeme(r";")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '*' => {
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::Star)
                        .lexeme(r"*")
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            '!' => {
                if self.match_char('=') {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::BangEqual)
                            .lexeme(r"!=")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Bang)
                            .lexeme(r"!")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                }
                Ok(())
            }
            '=' => {
                if self.match_char('=') {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::EqualEqual)
                            .lexeme(r"==")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Equal)
                            .lexeme(r"=")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                }
                Ok(())
            }
            '<' => {
                if self.match_char('=') {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::LessEqual)
                            .lexeme(r"<=")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Less)
                            .lexeme(r"<")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                }
                Ok(())
            }
            '>' => {
                if self.match_char('=') {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::GreaterEqual)
                            .lexeme(r">=")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Greater)
                            .lexeme(r">")
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                }
                Ok(())
            }
            '/' => {
                // If the next character is '/', the entire line is ignored.
                if self.match_char('/') {
                    while self.code_chars.peek().map(|&c| c).unwrap() != NEWLINE
                        && !self.is_at_end()
                    {
                        self.current_ptr += 1;
                        self.code_chars.next();
                    }
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Slash)
                            .lexeme(r"/")
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                }
                Ok(())
            }
            _ => Err(ParserError {}),
        }
    }
}
