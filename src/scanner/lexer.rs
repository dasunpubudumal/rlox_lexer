use std::vec;

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
                        .lexeme(String::from("("))
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
                        .lexeme(String::from(")"))
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
                        .lexeme(String::from("{"))
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
                        .lexeme(String::from("}"))
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
                        .lexeme(String::from(","))
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
                        .lexeme(String::from("."))
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
                        .lexeme(String::from("-"))
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
                        .lexeme(String::from("+"))
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
                        .lexeme(String::from(";"))
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
                        .lexeme(String::from("*"))
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
                            .lexeme(String::from("!="))
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Bang)
                            .lexeme(String::from("!"))
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
                            .lexeme(String::from("=="))
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Equal)
                            .lexeme(String::from("="))
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
                            .lexeme(String::from("<="))
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Less)
                            .lexeme(String::from("<"))
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
                            .lexeme(String::from(">="))
                            .line(line)
                            .literal(None)
                            .build(),
                    );
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Greater)
                            .lexeme(String::from(">"))
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
                    self.seek_until(NEWLINE);
                } else {
                    self.tokens.push(
                        TokenBuilder::new()
                            .kind(TokenType::Slash)
                            .lexeme(String::from("/"))
                            .line(line)
                            .literal(None)
                            .build(),
                    )
                }
                Ok(())
            }
            ' ' | '\t' | '\r' => Ok(()),
            '\n' => {
                self.current_line += 1;
                Ok(())
            }
            '"' => {
                // Iterate until the cursor meets the closing quotes
                // This loop will terminate when either if 1) closing quotes are met 2) cursor reaches end of code string
                let mut vector: Vec<char> = vec![];

                loop {
                    match self.code_chars.peek().map(|&c| c) {
                        Some(val) => {
                            if val == '"' {
                                self.current_ptr += 1;
                                self.code_chars.next();
                                break;
                            }
                            if val == NEWLINE {
                                self.current_line += 1;
                                break;
                            } else {
                                self.seek_with_add(&mut vector);
                            }
                        }
                        _ => break,
                    }
                }

                // If closing quote is not found before eof,
                if self.is_at_end() {
                    return Err(ParserError {
                        msg: format!(
                            "Unterminated string at line: {}, column: {}",
                            self.current_line, self.current_ptr
                        ),
                    });
                }

                // If closing quote is found before eof,
                let string = String::from_iter(vector.iter());
                self.tokens.push(
                    TokenBuilder::new()
                        .kind(TokenType::String)
                        .lexeme(string)
                        .line(line)
                        .literal(None)
                        .build(),
                );
                Ok(())
            }
            _ => Err(ParserError {
                msg: format!(
                    "Unrecognized token: {:?} at line {} column {}",
                    character, self.current_line, self.current_ptr
                ),
            }),
        }
    }
}
