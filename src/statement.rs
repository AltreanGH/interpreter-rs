use crate::token::Operation;

pub enum Statement {
    Assignment {
        output: usize,
        input: usize,
        operation: Operation,
        constant: usize,
    },
    While {
        variable: usize,
        content: Vec<Statement>,
    },
    Loop {
        count: usize,
        content: Vec<Statement>,
    }
}