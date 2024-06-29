use std::vec;

use crate::token::{self, Literal, LiteralType, TokenBuilder, TokenType};

use super::Scanner;

use crate::constants::{KEYWORDS, NEWLINE};
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

    /// Checks whether a given character is a digit (numerical digit)
    pub(crate) fn is_digit(&self, character: &char) -> bool {
        match character.to_digit(10) {
            Some(val) => val <= 9,
            _ => false,
        }
    }

    /// Checks a partial number value
    /// e.g. if 126.32 is the number, it checks 126 and 32 separately
    pub(crate) fn partial_number(&mut self, nvector: &mut Vec<char>) {
        loop {
            match self.code_chars.peek().map(|&c| c) {
                Some(val) => {
                    if self.is_digit(&val) {
                        self.current_ptr += 1;
                        match self.code_chars.next() {
                            Some(nchar) => {
                                nvector.push(nchar);
                            }
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    /// Check if the character is a alphabetic character
    pub(crate) fn is_alpha(&self, character: &char) -> bool {
        character.is_alphabetic()
    }

    /// Check numbers
    /// e.g. 126.32
    fn number(&mut self, current_char: char) {
        // while (isDigit(peek())) advance();
        // 1. Peek the next character
        // 2. Check whether if it's a digit
        // 3. If so, continue advancing until the next character is not a character
        // 4. If we meet a dot, we need to break.
        let mut nvector: Vec<char> = vec![current_char];
        self.partial_number(&mut nvector);

        // Look for a fractional part.
        match self.code_chars.peek().map(|&c| c) {
            Some(character) => {
                // Looking for the fractional part
                if character == '.' {
                    // Consume the "."
                    nvector.push(character);
                    self.seek();
                    match self.code_chars.peek().map(|&c| c) {
                        Some(next_char) => {
                            if self.is_digit(&next_char) {
                                self.partial_number(&mut nvector);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        let string_value = String::from_iter(nvector);

        self.tokens.push(
            TokenBuilder::<LiteralType>::new()
                .kind(TokenType::Number)
                .line(self.current_line)
                .lexeme(string_value.clone())
                .literal(Some(Literal {
                    kind: LiteralType::Float(string_value.parse::<f64>().unwrap()),
                }))
                .build(),
        )
    }

    fn string(&mut self, line: usize) -> Result<(), ParserError> {
        let mut vector: Vec<char> = vec![];

        loop {
            match self.code_chars.peek().map(|&c| c) {
                Some(val) => {
                    if val == '"' {
                        self.seek();
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
                _msg: format!(
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

    fn prepare_lexeme_for_id(&self, identifier: String) -> (TokenType, String) {
        let id_type = KEYWORDS
            .get(identifier.as_ref())
            .unwrap_or_else(|| &TokenType::Identifier);
        (*id_type, identifier)
    }

    pub(crate) fn identifier(&mut self, line: usize, prev_char: char) {
        let mut id_vec: Vec<char> = vec![];
        id_vec.push(prev_char);
        loop {
            match self.code_chars.peek().map(|&c| c) {
                Some(val) => {
                    if self.is_alpha(&val) {
                        self.seek_with_add(&mut id_vec);
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }
        let string = String::from_iter(id_vec.iter());
        let (token_type, lexeme) = self.prepare_lexeme_for_id(string);
        self.tokens.push(
            TokenBuilder::new()
                .kind(token_type)
                .lexeme(lexeme.to_string())
                .line(line)
                .literal(None)
                .build(),
        );
    }

    /// Scans individual characters and returns a token
    pub(crate) fn scan_individual_token(
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
            &NEWLINE => {
                self.current_line += 1;
                Ok(())
            }
            '"' => {
                // Iterate until the cursor meets the closing quotes
                // This loop will terminate when either if 1) closing quotes are met 2) cursor reaches end of code string
                self.string(line)
            }
            _ => {
                if self.is_digit(character) {
                    self.number(*character);
                    Ok(())
                } else if self.is_alpha(&character) {
                    self.identifier(line, *character);
                    Ok(())
                } else {
                    Err(ParserError {
                        _msg: format!(
                            "Unrecognized token: {:?} at line {} column {}",
                            character, self.current_line, self.current_ptr
                        ),
                    })
                }
            }
        }
    }
}

// Tests here are for testing tests listed in src/scanner/lexer.rs
#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[test]
    fn test_is_digit() {
        let string = String::from("1231.12");
        let scanner = Scanner::new(&string);
        let truth_value = scanner.is_digit(&'3');
        assert!(truth_value);
    }

    #[test]
    fn test_partial_number() {
        let string = String::from("126.32");
        let mut scanner = Scanner::new(&string);
        let mut resulting_vector: Vec<char> = vec![];
        scanner.partial_number(&mut resulting_vector);
        let result = String::from_iter(resulting_vector);
        assert_eq!(result, "126");
    }

    #[rstest]
    #[case('a', true)]
    #[case('!', false)]
    #[case('3', false)]
    fn test_is_alpha(#[case] input: char, #[case] expected: bool) {
        let mut string = String::from("test");
        string.push('\n');

        let scanner = Scanner::new(&string);
        let result = scanner.is_alpha(&input);

        assert_eq!(result, expected);
    }
}
