#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    ILLEGAL,
    EOF,
    IDENT(&'a str),
    INT(&'a str),

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

impl<'a> Token<'a> {
    pub fn identifier(&self) -> String {
        todo!();
    }

    pub fn token_type(&self) -> &str {
        match self {
            Token::ILLEGAL => "ILLEGAL",
            Token::EOF => "EOF",
            Token::IDENT(value) => *value,
            Token::INT(value) => *value,
            Token::ASSIGN => "=",
            Token::PLUS => "+",
            Token::MINUS => "-",
            Token::COMMA => ",",
            Token::SEMICOLON => ";",
            Token::LPAREN => "(",
            Token::RPAREN => ")",
            Token::LBRACE => "{",
            Token::RBRACE => "}",
            Token::FUNCTION => "FUNCTION",
            Token::LET => "LET",
            Token::BANG => "!",
            Token::ASTERISK => "*",
            Token::SLASH => "/",
            Token::LT => "<",
            Token::GT => ">",
            Token::IF => "IF",
            Token::ELSE => "ELSE",
            Token::RETURN => "RETURN",
            Token::TRUE => "TRUE",
            Token::FALSE => "FALSE",
            Token::EQ => "==",
            Token::NOTEQ => "!=",
        }
    }
}
