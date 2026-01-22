use std::{iter::Peekable, vec::IntoIter};

use crate::{statement::Statement, token::Token};

pub struct Parser {
    tokens: Peekable<IntoIter<Token<usize>>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token<usize>>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stats = Vec::new();
        while let Some(_) = self.tokens.peek() {
            stats.push(self.parse_statement());
        };
        stats
    }

    fn parse_statement(&mut self) -> Statement {
        todo!()
    }
}
