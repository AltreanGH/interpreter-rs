#[derive(Debug)]
pub enum Token<T> {
    DO,
    END,
    WHILE,
    LOOP,
    IN,
    OUT,
    ASSIGN,
    PLUS,
    MINUS,
    NUM { value: usize },
    VAR { name: T },
}

impl<T> Token<T> {
    pub fn map<U, F>(self, f: F) -> Token<U> where F: FnOnce(T) -> U {
        match self {
            Token::VAR { name } => Token::VAR { name: f(name) },
            Token::DO => Token::DO,
            Token::END => Token::END,
            Token::WHILE => Token::WHILE,
            Token::LOOP => Token::LOOP,
            Token::IN => Token::IN,
            Token::OUT => Token::OUT,
            Token::ASSIGN => Token::ASSIGN,
            Token::PLUS => Token::PLUS,
            Token::MINUS => Token::MINUS,
            Token::NUM { value } => Token::NUM { value },
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
            "+" => Token::PLUS,
            "-" => Token::MINUS,
            token => {
                if let Ok(val) = token.parse::<usize>() {
                    Token::NUM { value: val }
                } else if token.chars().all(|c| c.is_ascii_alphanumeric()) {
                    Token::VAR {
                        name: token.to_string(),
                    }
                } else {
                    return Err(format!("cannot parse token: {token}"));
                }
            }
        };
        Ok(res)
    }
}