use crate::token::Token;

pub fn lex(code: &str) -> Vec<Token<String>> {
    code.split_ascii_whitespace()
        .map(|token| Token::try_from(token).unwrap())
        .collect()
    // TODO remove unwrap
    // TODO is it faster to map all vars at the end of inside the iterator?
    // TODO map Token<String> to Token<usize> (indizes) and create the var array
}
