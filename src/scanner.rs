use std::char;
use std::str::Chars;
use crate::Token; 
use crate::TokenType;

/// Code is a reference. Current and previous tokens are returned and therefore not referred.
pub struct Scanner<'a> {
    pub code: &'a str,
    pub current_token: Option<Token>,
    pub previous_token: Option<Token>,
    pub current_line: usize,
    pub current_ptr: usize,
    pub previous_char: Option<char>,
    pub code_chars: Chars<'a>,
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
            current_line: 1,
            current_ptr: 0,
            previous_char: None,
            code_chars: code.chars(),
        }
    }

    /// Returns an iterator that contains tokens of type `Token`.
    /// Chained token is the EOF token that makes parsing a little easier.
    /// This is not an associated function, as it does have `self` in it. This needs to be called
    /// as a method.
    pub fn scan_tokens(&'a mut self) -> impl Iterator<Item = Token> + 'a {
        std::iter::from_fn(move || {
            let current_character = self.code_chars.next();
            let result = match current_character {
                Some(character) => self.scan_individual_token(&&character, self.current_line),
                None => None
            };
            self.current_ptr += 1;
            self.previous_char = current_character;
            result
        })
    }

    /// Advances the cursor
    fn advance(&mut self) -> Option<char> {
        self.code_chars.next()
    }

    /// Scans individual characters and returns a token
    fn scan_individual_token(&mut self, character: &char, line: usize) -> Option<Token> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_token() {
        let mut scanner = Scanner::new("{)");
        let tokens = scanner.scan_tokens().collect::<Vec<Token>>();
        assert!(tokens.get(0).is_some());
        assert_eq!(tokens.get(0).unwrap().lexeme, String::from("{"));
        assert_eq!(tokens.get(1).unwrap().lexeme, String::from(")"));
    }

}
