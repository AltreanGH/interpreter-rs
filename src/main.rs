mod lexer;
mod statement;
mod token;
mod parser;

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
    let t = lexer::lex(code);
    let a = parser::map_vars(t);
    parser::parse(a.0);
    return;
}