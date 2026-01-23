use crate::{
    statement::Statement,
    token::{Iteration, Operation},
};

pub fn interpret(stats: Vec<Statement>, mut vars: Vec<usize>) -> Vec<usize> {
    interpret_level(&stats, &mut vars);
    vars
}

fn interpret_level(stats: &Vec<Statement>, vars: &mut Vec<usize>) {
    for statement in stats.iter() {
        match statement {
            Statement::Assignment {
                output,
                input,
                operation,
                constant,
            } => vars[*output] = interpret_assignment(vars[*input], constant, operation),
            Statement::Iteration {
                variable,
                content,
                kind,
            } => interpret_iteration(*variable, content, vars, kind),
        }
    }
}

fn interpret_assignment(input: usize, constant: &usize, operation: &Operation) -> usize {
    match operation {
        Operation::PLUS => input + constant,
        Operation::MINUS => input - constant, // TODO panic when would be negative
    }
}

fn interpret_iteration(
    variable: usize,
    content: &Vec<Statement>,
    vars: &mut Vec<usize>,
    kind: &Iteration,
) {
    match kind {
        Iteration::LOOP => {
            let iterations = vars[variable];
            for _ in 0..iterations {
                interpret_level(content, vars);
            }
        }
        Iteration::WHILE => todo!(),
    };
}
