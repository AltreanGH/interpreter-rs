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
            Some(token) => panic!(
                "Expected ASSIGN after VAR({output}), but found: {:?}",
                token
            ),
            None => panic!("Unexpected end of input while expecting ASSIGN after VAR({output})"),
        };
        let input = match self.tokens.next() {
            Some(Token::VAR(i)) => i,
            Some(token) => panic!(
                "Expected VAR after ASSIGN in operation, but found: {:?}",
                token
            ),
            None => panic!("Unexpected end of input while expecting VAR after ASSIGN in operation"),
        };
        let operation = match self.tokens.next() {
            Some(Token::OP(op)) => op,
            Some(token) => panic!("Expected OP after VAR({input}), but found: {:?}", token),
            None => panic!("Unexpected end of input while expecting OP after VAR({input})"),
        };
        let constant = match self.tokens.next() {
            Some(Token::NUM(i)) => i,
            Some(token) => panic!(
                "Expected NUM after OP({operation:?}), but found: {:?}",
                token
            ),
            None => panic!("Unexpected end of input while expecting NUM after OP({operation:?})"),
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
            Some(token) => panic!("Expected VAR in iteration, but found: {:?}", token),
            None => panic!("Unexpected end of input while expecting VAR in iteration"),
        };
        match self.tokens.next() {
            Some(Token::DO) => {}
            Some(token) => panic!("Expected DO after VAR({variable}), but found: {:?}", token),
            None => panic!("Unexpected end of input while expecting DO after VAR({variable})"),
        };
        let content = self.parse();
        match self.tokens.next() {
            Some(Token::END) => {}
            Some(token) => panic!(
                "Expected END after iteration content, but found: {:?}",
                token
            ),
            None => panic!("Unexpected end of input while expecting END after iteration content"),
        };
        Statement::Iteration {
            variable,
            content,
            kind,
        }
    }
}

fn unexpected_message(unexpected: Option<Token<usize>>) -> String {
    format!("Unexpected token '{:?}'", unexpected)
}
