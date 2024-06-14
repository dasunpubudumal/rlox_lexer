mod lexer;

use log::info;

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
    pub tokens: Vec<Token>,
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

    use crate::token::TokenType;

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
        string.push('\n');
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert!(tokens.is_empty())
    }

    #[test]
    fn test_scan_token_for_slash() {
        let mut string = String::from("/!");
        string.push('\n');
        let scanner = Scanner::new(&string);
        let tokens = scanner.scan_tokens().tokens;
        debug!("Tokens: {:?}", tokens);
        assert_eq!(tokens.get(0).unwrap().kind, TokenType::Slash);
    }
}
