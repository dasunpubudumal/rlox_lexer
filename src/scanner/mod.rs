mod lexer;

use log::{debug, info};

use crate::token::LiteralType;
use crate::Token;
use std::char;
use std::iter::Peekable;
use std::str::Chars;

/// Code is a reference. Current and previous tokens are returned and therefore not referred.
pub struct Scanner<'a> {
    pub code: &'a str,
    pub current_line: usize,
    pub current_ptr: usize,
    pub previous_char: Option<char>,
    pub code_chars: Peekable<Chars<'a>>,
    pub tokens: Vec<Token<LiteralType>>,
}

/// We need to guarantee that the reference `code` we provide into `new()` lives throughout the Scanner instance.
/// Hence, the lifetime `'a`.
impl<'a> Scanner<'a> {
    /// Creates a new Scanner struct
    pub fn new(code: &'a str) -> Scanner<'a> {
        Scanner {
            code,
            current_line: 1,
            current_ptr: 0,
            previous_char: None,
            code_chars: code.chars().peekable(),
            tokens: vec![],
        }
    }

    /// Second character lookahead for parsing decimal points

    /// Reference: https://craftinginterpreters.com/scanning.html#number-literals
    pub fn peek_next(&mut self) -> Option<&char> {
        if self.current_ptr + 1 >= self.code.len() {
            return Some(&'\0');
        }
        self.code_chars.peek()
    }

    /// Seeks the code string by one character.
    pub fn seek(&mut self) {
        self.current_ptr += 1;
        self.code_chars.next();
    }

    /// Seek with adding the current character to a given vector
    pub fn seek_with_add(&mut self, vector: &mut Vec<char>) {
        self.current_ptr += 1;
        match self.code_chars.next() {
            Some(c) => {
                (*vector).push(c);
            }
            _ => {}
        }
    }

    /// Seek until a certail terminal_char character.
    pub fn seek_until(&mut self, terminal_char: char) {
        while !self.is_at_end() && self.code_chars.peek().map(|&c| c).unwrap() != terminal_char {
            self.seek();
        }
    }

    /// Returns an iterator that contains tokens of type `Token`.
    /// Chained token is the EOF token that makes parsing a little easier.
    /// This is not an associated function, as it does have `self` in it. This needs to be called
    /// as a method. scan_tokens() takes in an exclusive reference (i.e., mut self) to the instance of Scanner
    /// because there's no need to invoke any other scanner functions after the invocation of this function.
    pub fn scan_tokens(mut self) -> Self {
        while !self.is_at_end() {
            let current_character = self.code_chars.next();
            match current_character {
                Some(character) => {
                    match self.scan_individual_token(&character, self.current_line) {
                        Ok(()) => (),
                        Err(error) => {
                            info!("Error: {:?}", error);
                        }
                    }
                }
                None => (),
            }
            self.current_ptr += 1;
            self.previous_char = current_character;
        }
        self
    }

    /// Checks if the cursor is at end
    pub(crate) fn is_at_end(&self) -> bool {
        return self.current_ptr >= self.code.len();
    }
}

#[cfg(test)]
mod tests {
    use log::debug;

    use crate::{
        constants::NEWLINE,
        token::{Literal, TokenType},
    };

    use super::*;

    #[ctor::ctor]
    fn init() {
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_scan_token_for_individual_tokens() {
        let scanner = Scanner::new("{)");
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert!(tokens.get(0).is_some());
        assert_eq!(tokens.get(0).unwrap().lexeme, String::from("{"));
        assert_eq!(tokens.get(1).unwrap().lexeme, String::from(")"));
    }

    #[test]
    fn test_scan_token_for_operators_scenario_1() {
        let scanner = Scanner::new("!=");
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap().kind, TokenType::BangEqual);
    }

    #[test]
    fn test_scan_token_for_operators_scenario_2() {
        let scanner = Scanner::new("!!=");
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.first().unwrap().kind, TokenType::Bang);
        assert_eq!(tokens.get(1).unwrap().kind, TokenType::BangEqual);
    }

    #[test]
    fn test_scan_token_for_comment() {
        let mut string = String::from("// hello world");
        string.push(NEWLINE);
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert!(tokens.is_empty())
    }

    #[test]
    fn test_scan_token_for_slash() {
        let mut string = String::from("/!");
        string.push(NEWLINE);
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.get(0).unwrap().kind, TokenType::Slash);
        assert_eq!(tokens.get(1).unwrap().kind, TokenType::Bang);
    }

    #[test]
    fn test_scan_token_for_operators() {
        let mut string = String::from("!*+-/=<> <= ==");
        string.push(NEWLINE);
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 10);
    }

    #[test]
    fn test_scan_for_skipped_tokens() {
        let mut string = String::from("\t >=");
        string.push(NEWLINE);
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap().kind, TokenType::GreaterEqual);
    }

    #[test]
    fn test_string_tokens() {
        let mut string = String::from("\" Hello World!\"");
        string.push(NEWLINE);
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap().kind, TokenType::String);
        assert_eq!(tokens.first().unwrap().lexeme, " Hello World!");
    }

    // #[ignore]
    #[test]
    fn test_numbers() {
        let mut string = String::from("126.32");
        string.push(NEWLINE);
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap().kind, TokenType::Number);
        assert_eq!(
            tokens.first().unwrap().literal.as_ref().unwrap(),
            &Literal {
                kind: LiteralType::Float(126.32)
            }
        );
    }
}
