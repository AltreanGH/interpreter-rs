use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

mod lexer;
mod parser;
mod statement;
mod token;
mod interpreter;

fn main() {
    let code = "IN n
OUT factorial

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
    let p = Parser::new(l.0).parse();
    let vars = vec![0; l.1.len()];
    Interpreter::new(vars, p).interpret();
    return;
}
