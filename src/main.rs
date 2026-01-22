mod lexer;
mod statement;
mod token;

fn main() {
    let t = lexer::lex(" DO 03 3 fl:=k=d+sjf2 LOOP WHILE dDdO d DO o");
    for c in t {
        println!("{:#?}", c);
    }
    return;
}