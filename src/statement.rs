use crate::token::{Iteration, Operation};

pub enum Statement {
    Assignment {
        output: usize,
        input: usize,
        operation: Operation,
        constant: usize,
    },
    Iteration {
        variable: usize,
        content: Vec<Statement>,
        kind: Iteration,
    },
}
