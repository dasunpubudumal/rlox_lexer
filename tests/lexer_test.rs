use log::debug;
use rstest::{fixture, rstest};

use rlox_lexer::{constants::NEWLINE, scanner::Scanner, Literal, LiteralType, Token, TokenType};

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
    assert_eq!(tokens.first().unwrap().kind, TokenType::BangEqual);
}

#[test]
fn test_scan_token_for_operators_scenario_2() {
    let scanner = Scanner::new("!!=");
    let tokens = scanner.scan_tokens().tokens;
    debug!("Tokens: {:?}", tokens);
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
}

#[test]
fn test_scan_for_skipped_tokens() {
    let mut string = String::from("\t >=");
    string.push(NEWLINE);
    let scanner = Scanner::new(&string);
    let tokens = scanner.scan_tokens().tokens;
    debug!("Tokens: {:?}", tokens);
    assert_eq!(tokens.first().unwrap().kind, TokenType::GreaterEqual);
}

#[test]
fn test_string_tokens() {
    let mut string = String::from("\" Hello World!\"");
    string.push(NEWLINE);
    let scanner = Scanner::new(&string);
    let tokens = scanner.scan_tokens().tokens;
    debug!("Tokens: {:?}", tokens);
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
    assert_eq!(tokens.first().unwrap().kind, TokenType::Number);
    assert_eq!(
        tokens.first().unwrap().literal.as_ref().unwrap(),
        &Literal {
            kind: LiteralType::Float(126.32)
        }
    );
}

#[test]
fn test_identifier() {
    let mut string = String::from("for");
    string.push('\n');
    let scanner = Scanner::new(&string);
    let tokens = scanner.scan_tokens().tokens;
    debug!("Tokens: {:?}", tokens);
    assert_eq!(tokens.first().unwrap().kind, TokenType::Identifier);
    assert_eq!(tokens.first().unwrap().lexeme, "for");
}

#[rstest]
#[case("!*+-/=<> <= ==", 10)]
#[case("\t >= ", 1)]
#[case("\" Hello World!\"", 1)]
#[case("2 + 2.1", 3)]
#[case("for if", 2)]
fn test_scan_tokens(#[case] input: &str, #[case] expected_len: usize) {
    let mut string = String::from(input);
    string.push('\n');

    let scanner = Scanner::new(&string);
    let tokens: Vec<Token<LiteralType>> = scanner.scan_tokens().tokens;

    debug!("Tokens: {:?}", tokens);
    assert_eq!(tokens.len(), expected_len);
}
