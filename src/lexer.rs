use std::collections::HashMap;

use crate::token::Token;

pub struct Lexer<'a> {
    code: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self { code }
    }

    pub fn lex(&self) -> (Vec<Token<usize>>, HashMap<String, usize>) {
        self.map_vars(
            self.code
                .split_ascii_whitespace()
                .map(|token| Token::try_from(token).unwrap())
                .collect(),
        )
        // TODO remove unwrap
        // TODO is it faster to map all vars at the end of inside the iterator?
        // TODO map Token<String> to Token<usize> (indizes) and create the var array
    }

    fn map_vars(&self, tokens: Vec<Token<String>>) -> (Vec<Token<usize>>, HashMap<String, usize>) {
        let mut name_map = HashMap::new();
        let mut mapped_tokens = Vec::with_capacity(tokens.len());

        for token in tokens {
            mapped_tokens.push(token.map(|name| {
                let i = name_map.len();
                *name_map.entry(name).or_insert(i)
            }));
        }
        (mapped_tokens, name_map)
    }
}
