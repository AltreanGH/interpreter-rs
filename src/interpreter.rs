use std::collections::HashMap;

use crate::{
    statement::Statement,
    token::{Iteration, Operation},
};

pub fn interpret(
    stats: Vec<Statement>,
    var_mapping: HashMap<String, usize>,
    mut vars: Vec<usize>,
) -> HashMap<String, usize> {
    interpret_level(&stats, &mut vars);
    unmap_vars(vars, var_mapping)
}

fn interpret_level(stats: &Vec<Statement>, vars: &mut Vec<usize>) {
    for statement in stats.iter() {
        match statement {
            Statement::Assignment {
                output,
                op1,
                operation,
                op2,
            } => {
                vars[*output] =
                    interpret_assignment(op1.to_value(vars), operation, op2.to_value(vars))
            }
            Statement::Iteration {
                variable,
                content,
                kind,
            } => interpret_iteration(*variable, content, vars, kind),
        }
    }
}

fn interpret_assignment(op1: usize, operation: &Operation, op2: usize) -> usize {
    match operation {
        Operation::ADD => op1 + op2,
        Operation::SUB => op1.checked_sub(op2).unwrap_or(0),
        Operation::MUL => todo!(),
        Operation::DIV => todo!(),
        Operation::MOD => todo!(),
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
            if content.len() == 1
                && let Statement::Assignment {
                    output,
                    op1,
                    operation,
                    op2,
                } = &content[0]
                && vars[*output] == op1.to_value(vars)
            {
                vars[*output] = interpret_assignment(
                    op1.to_value(vars),
                    operation,
                    op2.to_value(vars) * vars[variable],
                );
            } else {
                let iterations = vars[variable];
                for _ in 0..iterations {
                    interpret_level(content, vars);
                }
            }
        }
        Iteration::WHILE => {
            while vars[variable] != 0 {
                interpret_level(content, vars);
            }
        }
    };
    // LOOP i DO
    //   a = a + 2
    // END
    // a = 2i
    // TODO optional optimization step
    // TODO tests
}

fn unmap_vars(vars: Vec<usize>, mut var_mapping: HashMap<String, usize>) -> HashMap<String, usize> {
    var_mapping.values_mut().for_each(|v| *v = vars[*v]);
    var_mapping
}
