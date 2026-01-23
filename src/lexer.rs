use std::collections::HashMap;

use crate::token::Token;

pub struct Lexer<'a> {
    code: &'a str,
}

pub struct LexingResult {
    pub tokens: Vec<Token<usize>>,
    pub var_mapping: HashMap<String, usize>,
    pub vars: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self { code }
    }

    pub fn lex(&self) -> LexingResult {
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

    fn map_vars(&self, tokens: Vec<Token<String>>) -> LexingResult {
        let mut var_mapping = HashMap::new();
        let mut mapped_tokens = Vec::with_capacity(tokens.len());

        for token in tokens {
            mapped_tokens.push(token.map(|name| {
                let i = var_mapping.len();
                *var_mapping.entry(name).or_insert(i)
            }));
        }

        let var_num = var_mapping.len();
        LexingResult {
            tokens: mapped_tokens,
            var_mapping,
            vars: vec![0; var_num],
        }
    }
}
