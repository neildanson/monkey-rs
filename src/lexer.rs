use crate::token::Token;

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
        if self.read_position > self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
            self.position = self.read_position;
            self.read_position += 1;
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    fn is_letter(ch: Option<char>) -> bool {
        ch.map(|c| c.is_alphabetic() && !c.is_whitespace())
            .unwrap_or(false)
    }

    fn is_whitespace(ch: Option<char>) -> bool {
        ch.map(|c| c.is_whitespace()).unwrap_or(false)
    }
    fn is_digit(ch: Option<char>) -> bool {
        ch.map(|c| c.is_numeric()).unwrap_or(false)
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn skip_whitespace(&mut self) {
        while Self::is_whitespace(self.ch) {
            self.read_char()
        }
    }

    fn lookup_identifier(ident: &str) -> Token {
        match ident {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            ident => Token::IDENT(ident.to_string()),
        }
    }

    //TODO Seems vaguely Iterable?
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();

                    Token::NOTEQ
                } else {
                    Token::BANG
                }
            }
            Some(';') => Token::SEMICOLON,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some(',') => Token::COMMA,
            Some('+') => Token::PLUS,
            Some('-') => Token::MINUS,
            Some('{') => Token::LBRACE,
            Some('}') => Token::RBRACE,
            Some('*') => Token::ASTERISK,
            Some('/') => Token::SLASH,
            Some('<') => Token::LT,
            Some('>') => Token::GT,
            Some(_) => {
                //Some ugly early returns - beware
                if Self::is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return Self::lookup_identifier(&ident);
                } else if Self::is_digit(self.ch) {
                    return Token::INT(self.read_number());
                } else {
                    Token::ILLEGAL //, format!("{:?}", self.ch))
                }
            }
            None => Token::EOF,
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_next_token_int() {
        let input = "xyz";
        let expected = vec![Token::IDENT("xyz".to_string())];
        let mut lexer = super::Lexer::new(input.to_string());

        for token in expected.iter() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token);
        }
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = super::Lexer::new(input.to_string());

        for token in expected.iter() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token);
        }
    }

    #[test]
    fn test_next_monkey_example() {
        let monkey_example = "
let five = 5;\
let ten = 10;\
let add = fn(x, y) {\
     x + y;\
}; 
let result = add(five, ten);";
        let mut lexer = super::Lexer::new(monkey_example.to_string());
        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];

        for (pos, token) in expected.iter().enumerate() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token, "Failed at position: {}", pos);
        }
    }

    #[test]
    fn test_next_monkey_example_invalid_syntax() {
        let monkey_example = "
let five = 5;\
let ten = 10;\
let add = fn(x, y) {\
    x + y;\
};\
let result = add(five, ten);\
!-/*5; 5 < 10 > 5;";
        let mut lexer = super::Lexer::new(monkey_example.to_string());
        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::GT,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::EOF,
        ];

        for (pos, token) in expected.iter().enumerate() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token, "Failed at position: {}", pos);
        }
    }

    #[test]
    fn test_next_monkey_example_extra_keywords() {
        let monkey_example = "
let five = 5;\
let ten = 10;\
let add = fn(x, y) {\
    x + y;\
};\
let result = add(five, ten);\
!-/*5; 5 < 10 > 5;\
if (5 < 10) {\
    return true;\
} else { \
    return false;\
}";
        let mut lexer = super::Lexer::new(monkey_example.to_string());
        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::GT,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::EOF,
        ];

        for (pos, token) in expected.iter().enumerate() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token, "Failed at position: {}", pos);
        }
    }

    #[test]
    fn test_next_monkey_example_extra_keywords_2() {
        let monkey_example = "
let five = 5;\
let ten = 10;\
let add = fn(x, y) {\
    x + y;\
};\
let result = add(five, ten);\
!-/*5; 5 < 10 > 5;\
if (5 < 10) {\
    return true;\
} else { \
    return false;\
}
10 == 10;
10 != 9;
";
        let mut lexer = super::Lexer::new(monkey_example.to_string());
        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::GT,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT("10".to_string()),
            Token::EQ,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::INT("10".to_string()),
            Token::NOTEQ,
            Token::INT("9".to_string()),
            Token::SEMICOLON,
            Token::EOF,
        ];

        for (pos, token) in expected.iter().enumerate() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token, "Failed at position: {}", pos);
        }
    }
}
