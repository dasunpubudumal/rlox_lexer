use crate::Token;
use crate::TokenType;

/// Code is a reference. Current and previous tokens are returned and therefore not referred.
pub struct Scanner<'a> {
    pub code: &'a str,
    pub current_token: Option<Token>,
    pub previous_token: Option<Token>
}

/// We need to guarantee that the reference `code` we provide into `new()` lives throughout the Scanner instance.
/// Hence, the lifetime `'a`.
impl<'a> Scanner<'a> {
    /// Creates a new Scanner struct
    pub fn new(code: &'a str) -> Scanner<'a> {
        Scanner {
            code,
            current_token: Some(Token {
                kind: TokenType::Start
            }),
            previous_token: None
        }
    }

    /// Returns the next token
    pub fn next_token(&mut self) -> Token {
        todo!("Yet to be implemented")
    }
}