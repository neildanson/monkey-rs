use crate::parser::token::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
            self.position = self.read_position;
            self.read_position += 1;
        }
    }

    //TODO Seems vaguely Iterable?
    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some(ch) if ch == '=' => Token::new(TokenType::ASSIGN, ch.to_string()),
            Some(ch) if ch == ';' => Token::new(TokenType::SEMICOLON, ch.to_string()),
            Some(ch) if ch == '(' => Token::new(TokenType::LPAREN, ch.to_string()),
            Some(ch) if ch == ')' => Token::new(TokenType::RPAREN, ch.to_string()),
            Some(ch) if ch == ',' => Token::new(TokenType::COMMA, ch.to_string()),
            Some(ch) if ch == '+' => Token::new(TokenType::PLUS, ch.to_string()),
            Some(ch) if ch == '{' => Token::new(TokenType::LBRACE, ch.to_string()),
            Some(ch) if ch == '}' => Token::new(TokenType::RBRACE, ch.to_string()),
            _ => Token::new(TokenType::EOF, "".to_string()),
        };
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected = vec![
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::LBRACE, "{".to_string()),
            Token::new(TokenType::RBRACE, "}".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
        ];
        let mut lexer = super::Lexer::new(input.to_string());
        for (i, token) in expected.iter().enumerate() {
            assert_eq!(*token, expected[i]);
            let _ = lexer.next_token();
        }
    }
}
