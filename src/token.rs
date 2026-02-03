#[derive(Debug, PartialEq)]
pub enum Token<T> {
    DO,
    END,
    ITER(Iteration),
    ASSIGN,
    OP(Operation),
    NUM(usize),
    VAR(T),
}

#[derive(Debug, PartialEq)]
pub enum Iteration {
    LOOP,
    WHILE,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
}

#[derive(Debug)]
pub enum Operand {
    CONST(usize),
    VAR(usize),
}

impl Operand {
    pub fn to_value(&self, vars: &Vec<usize>) -> usize {
        match self {
            Operand::CONST(val) => *val,
            Operand::VAR(val) => vars[*val],
        }
    }
}

impl<T> Token<T> {
    pub fn map<U, F>(self, f: F) -> Token<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Token::DO => Token::DO,
            Token::END => Token::END,
            Token::ITER(kind) => Token::ITER(kind),
            Token::ASSIGN => Token::ASSIGN,
            Token::OP(op) => Token::OP(op),
            Token::NUM(val) => Token::NUM(val),
            Token::VAR(val) => Token::VAR(f(val)),
        }
    }
}

impl TryFrom<&str> for Token<String> {
    type Error = String;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        let res = match token {
            "DO" => Token::DO,
            "END" => Token::END,
            "WHILE" => Token::ITER(Iteration::WHILE),
            "LOOP" => Token::ITER(Iteration::LOOP),
            ":=" => Token::ASSIGN,
            "+" => Token::OP(Operation::ADD),
            "-" => Token::OP(Operation::SUB),
            token => {
                if let Ok(val) = token.parse::<usize>() {
                    Token::NUM(val)
                } else if token.chars().all(|c| c.is_ascii_alphanumeric()) {
                    Token::VAR(token.to_string())
                } else {
                    return Err(format!("Cannot parse token '{token}'"));
                }
            }
        };
        Ok(res)
    }
}
