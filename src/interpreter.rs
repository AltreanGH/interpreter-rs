use crate::statement::Statement;

pub struct Interpreter {
    vars: Vec<usize>,
    stats: Vec<Statement>,
}

impl Interpreter {
    pub fn new(vars: Vec<usize>, stats: Vec<Statement>) -> Self {
        Self { vars, stats }
    }

    pub fn interpret(&self) -> Vec<usize> {
        todo!()
    }
}
