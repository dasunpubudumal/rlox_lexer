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

    /// Returns an iterator that contains tokens of type `Token`.
    /// Chained token is the EOF token that makes parsing a little easier.
    pub fn scan_tokens(&'a mut self) -> impl Iterator<Item = Token> + 'a {
        let current_line = self.current_line;
        std::iter::from_fn(move || {
            // This still moves `self` into the closure.
            // Figure out a way around this? `Rc` and `RefCell` might be good candiates to solve this.
            let next_token = self.next_token();
            if next_token.kind != TokenType::Eof {
                Some(next_token)
            } else {
                None
            }
        })
        .chain(std::iter::once(Token {
            kind: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line: current_line,
        }))
    }

    /// Returns the next token. This is a private function only used by `scan_tokens()` function
    fn next_token(&mut self) -> Token {
        todo!("Yet to be implemented")
    }
}
