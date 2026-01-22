pub enum Token<T> {
    DO,
    END,
    WHILE,
    LOOP,
    IN,
    OUT,
    ASSIGN,
    OP(Operation),
    NUM(usize),
    VAR(T)
}

pub enum Operation {
    PLUS,
    MINUS
}

impl<T> Token<T> {
    pub fn map<U, F>(self, f: F) -> Token<U> where F: FnOnce(T) -> U {
        match self {
            Token::VAR(val) => Token::VAR(f(val)),
            Token::DO => Token::DO,
            Token::END => Token::END,
            Token::WHILE => Token::WHILE,
            Token::LOOP => Token::LOOP,
            Token::IN => Token::IN,
            Token::OUT => Token::OUT,
            Token::ASSIGN => Token::ASSIGN,
            Token::OP(op) => Token::OP(op),
            Token::NUM(val) => Token::NUM(val),
        }
    }
}

impl TryFrom<&str> for Token<String> {
    type Error = String;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        let res = match token {
            "DO" => Token::DO,
            "END" => Token::END,
            "WHILE" => Token::WHILE,
            "LOOP" => Token::LOOP,
            "IN" => Token::IN,
            "OUT" => Token::OUT,
            ":=" => Token::ASSIGN,
            "+" => Token::OP(Operation::PLUS),
            "-" => Token::OP(Operation::MINUS),
            token => {
                if let Ok(val) = token.parse::<usize>() {
                    Token::NUM(val)
                } else if token.chars().all(|c| c.is_ascii_alphanumeric()) {
                    Token::VAR(token.to_string())
                } else {
                    return Err(format!("cannot parse token: {token}"));
                }
            }
        };
        Ok(res)
    }
}