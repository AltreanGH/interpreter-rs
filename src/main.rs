use std::{env, fs};

use crate::parser::Parser;

mod interpreter;
mod lexer;
mod parser;
mod statement;
mod token;

fn main() {
    let path: &str = &env::args().collect::<Vec<String>>()[1];
    let code = fs::read_to_string(path).unwrap();
    let lexed = lexer::lex(code);
    let parsed = Parser::new(lexed.tokens).parse();
    let res = interpreter::interpret(parsed, lexed.var_mapping, lexed.vars);
    for r in res {
        println!("{}, {}", r.0, r.1);
    }
    return;
}
