#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,
    MINUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,

    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    IF,
    ELSE,
    RETURN,

    TRUE,
    FALSE,

    EQ,
    NOTEQ,
}

impl Token {
    pub fn identifier(&self) -> String {
        todo!();
    }

    pub fn token_type(&self) -> String {
        match self {
            Token::ILLEGAL => "ILLEGAL".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::IDENT(value) => value.clone(),
            Token::INT(value) => value.clone(),
            Token::ASSIGN => "=".to_string(),
            Token::PLUS => "+".to_string(),
            Token::MINUS => "-".to_string(),
            Token::COMMA => ",".to_string(),
            Token::SEMICOLON => ";".to_string(),
            Token::LPAREN => "(".to_string(),
            Token::RPAREN => ")".to_string(),
            Token::LBRACE => "{".to_string(),
            Token::RBRACE => "}".to_string(),
            Token::FUNCTION => "FUNCTION".to_string(),
            Token::LET => "LET".to_string(),
            Token::BANG => "!".to_string(),
            Token::ASTERISK => "*".to_string(),
            Token::SLASH => "/".to_string(),
            Token::LT => "<".to_string(),
            Token::GT => ">".to_string(),
            Token::IF => "IF".to_string(),
            Token::ELSE => "ELSE".to_string(),
            Token::RETURN => "RETURN".to_string(),
            Token::TRUE => "TRUE".to_string(),
            Token::FALSE => "FALSE".to_string(),
            Token::EQ => "==".to_string(),
            Token::NOTEQ => "!=".to_string(),
        }
    }
}
