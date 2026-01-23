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
        match self.tokens.next().expect("unexpected EOF") {
            Token::ITER(kind) => self.parse_iteration(kind),
            Token::IN => todo!(),
            Token::OUT => todo!(),
            Token::VAR(var) => self.parse_operation(var),
            token => todo!(), // TODO panic unexpected token
        }
    }

    fn parse_operation(&mut self, output: usize) -> Statement {
        // TODO catch all missing tokens panics
        assert!(matches!(self.tokens.next(), Some(Token::ASSIGN)));
        let input = match self.tokens.next() {
            Some(Token::VAR(i)) => i,
            _ => todo!(),
        };
        let operation = match self.tokens.next() {
            Some(Token::OP(op)) => op,
            _ => todo!(),
        };
        let constant = match self.tokens.next() {
            Some(Token::NUM(i)) => i,
            _ => todo!(),
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
            _ => todo!(),
        };
        assert!(matches!(self.tokens.next(), Some(Token::DO)));
        let content = self.parse();
        assert!(matches!(self.tokens.next(), Some(Token::END)));
        Statement::Iteration {
            variable,
            content,
            kind,
        }
    }
}
