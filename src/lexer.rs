use std::collections::HashMap;

use crate::token::Token;

pub struct LexingResult {
    pub tokens: Vec<Token<usize>>,
    pub var_mapping: HashMap<String, usize>,
    pub vars: Vec<usize>,
}

pub fn lex(code: String) -> LexingResult {
    map_vars(
        code.split_ascii_whitespace()
            .map(|token| Token::try_from(token).unwrap())
            .collect(),
    )
}

fn map_vars(tokens: Vec<Token<String>>) -> LexingResult {
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
