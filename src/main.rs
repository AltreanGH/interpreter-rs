use crate::{lexer::Lexer, parser::Parser};

mod interpreter;
mod lexer;
mod parser;
mod statement;
mod token;

fn main() {
    let code = "
n := zero + 5
factorial := zero + 1
i := zero + 0

LOOP n DO
  i := i + 1
  
  oldFactorial := zero + 0
  LOOP factorial DO
    oldFactorial := oldFactorial + 1
  END
  
  factorial := zero + 0
  LOOP i DO
    LOOP oldFactorial DO
      factorial := factorial + 1
    END
  END
END";
    let l = Lexer::new(code).lex();
    let p = Parser::new(l.tokens).parse();
    let res = interpreter::interpret(p, l.vars); // TODO use l.var_mapping
    for r in res {
        println!("{}", r);
    }
    return;
}
