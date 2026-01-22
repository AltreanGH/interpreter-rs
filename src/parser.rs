use std::{collections::HashMap, vec};

use crate::{
    statement::Statement,
    token::{self, Token},
};

pub fn parse(lexed: Vec<Token<usize>>) -> Vec<Statement> {
    // TODO array von Tokens ergeben statement -> LOOP + VAR + DO + ... + END -> Loop
    // for token in lexed {
    //     match token {}
    // }
    vec![]
}

pub fn map_vars(tokens: Vec<Token<String>>) -> (Vec<Token<usize>>, HashMap<String, usize>) {
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
