use crate::token::{Iteration, Operand, Operation};

pub enum Statement {
    Assignment {
        output: usize,
        op1: Operand,
        operation: Operation,
        op2: Operand,
    },
    Iteration {
        variable: usize,
        content: Vec<Statement>,
        kind: Iteration,
    },
}
