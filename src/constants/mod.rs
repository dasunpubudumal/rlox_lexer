use crate::TokenType;
use phf::phf_map;

pub const NEWLINE: char = '\n';

/// https://crates.io/crates/phf
pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"       => TokenType::And,
    "class"     => TokenType::Class,
    "else"      => TokenType::Else,
    "false"     => TokenType::False,
    "for"       => TokenType::For,
    "fun"       => TokenType::Fun,
    "if"        => TokenType::If,
    "nil"       => TokenType::Nil,
    "or"        => TokenType::Print,
    "return"    => TokenType::Return,
    "super"     => TokenType::Super,
    "this"      => TokenType::This,
    "true"      => TokenType::True,
    "var"       => TokenType::Var,
    "while"     => TokenType::While,
};
