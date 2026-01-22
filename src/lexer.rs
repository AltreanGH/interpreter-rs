use crate::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    input
        .split_ascii_whitespace()
        .map(|token| Token::try_from(token).unwrap())
        .collect() // TODO remove unwrap
}
