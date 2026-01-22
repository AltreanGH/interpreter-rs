#[derive(Debug)]
pub enum Token {
    DO,
    END,
    WHILE,
    LOOP,
    IN,
    OUT,
    ASSIGN,
    PLUS,
    MINUS,
    NUM,
    VAR,
}

impl TryFrom<&str> for Token {
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
            t if t.chars().all(|c| c.is_numeric()) => Token::NUM,
            t if t.chars().all(|c| c.is_ascii_alphanumeric()) => Token::VAR,
            _ => return Err(format!("cannot parse token: {token}")),
        };
        Ok(res)
    }
}
