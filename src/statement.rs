use crate::token::Token;

pub enum Statement {
    Addition { constant: usize },
    Loop { content: Vec<Statement> },
}

trait Interpretable {
    fn interpret(&self, input: usize) -> usize;
}

impl Interpretable for Statement {
    fn interpret(&self, input: usize) -> usize {
        match self { // TODO extract all variables during parsing -> use arrays that are more efficient
            Statement::Addition { constant } => constant + input,
            Statement::Loop { content } => {
                // (0..input).for_each(|i| {
                //     content.for
                // });
                return 0;
            }
        }
    }
}

// impl TryFrom<&[Token]> for Statement {
//     type Error = String;

//     fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
//         match tokens[0] {
//             Token::WHILE => {
//                 let 
//             }
//             Token::LOOP => todo!(),
//             Token::IN => todo!(),
//             Token::OUT => todo!(),
//             Token::VAR => todo!(),
//             _ => todo!()
//         }
//     }
// }