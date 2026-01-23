use std::{iter::Peekable, vec::IntoIter};

use crate::{
    statement::Statement,
    token::{Iteration, Token},
};

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
        while let Some(token) = self.tokens.peek() {
            if matches!(token, Token::END) {
                break;
            }
            stats.push(self.parse_statement());
        }
        stats
    }

    fn parse_statement(&mut self) -> Statement {
        match self.tokens.next().expect("Unexpected EOF") {
            Token::ITER(kind) => self.parse_iteration(kind),
            Token::VAR(var) => self.parse_operation(var),
            token => panic!("{}", unexpected_message(Some(token))),
        }
    }

    fn parse_operation(&mut self, output: usize) -> Statement {
        match self.tokens.next() {
            Some(Token::ASSIGN) => {}
            token => panic!("{}", unexpected_message(token)),
        }
        let input = match self.tokens.next() {
            Some(Token::VAR(i)) => i,
            token => panic!("{}", unexpected_message(token)),
        };
        let operation = match self.tokens.next() {
            Some(Token::OP(op)) => op,
            token => panic!("{}", unexpected_message(token)),
        };
        let constant = match self.tokens.next() {
            Some(Token::NUM(i)) => i,
            token => panic!("{}", unexpected_message(token)),
        };
        Statement::Assignment {
            output,
            input,
            operation,
            constant,
        }
    }

    fn parse_iteration(&mut self, kind: Iteration) -> Statement {
        let variable = match self.tokens.next() {
            Some(Token::VAR(i)) => i,
            token => panic!("{}", unexpected_message(token)),
        };
        match self.tokens.next() {
            Some(Token::DO) => {}
            token => panic!("{}", unexpected_message(token)),
        }
        let content = self.parse();
        match self.tokens.next() {
            Some(Token::END) => {}
            token => panic!("{}", unexpected_message(token)),
        }
        Statement::Iteration {
            variable,
            content,
            kind,
        }
    }
}

fn unexpected_message(unexpected: Option<Token<usize>>) -> String {
    return format!("Unexpectded token '{:?}'", unexpected);
}
