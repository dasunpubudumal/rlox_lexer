use crate::Token;

pub struct Scanner<'a> {
    code: &'a str
}

/// We need to guarantee that the reference `code` we provide into `new()` lives throughout the Scanner instance.
/// Hence, the lifetime `'a`.
impl<'a> Scanner<'a> {
    /// Creates a new Scanner struct
    pub fn new(code: &'a str) -> Scanner<'a> {
        Scanner {
            code
        }
    }

    /// Scans tokens
    pub fn next_token(&mut self) -> Token {
        todo!("Yet to be implemented")
    }
}