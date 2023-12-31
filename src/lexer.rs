use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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
        self.input.chars().nth(self.read_position)
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

    fn read_identifier(&mut self) -> &'a str {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
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
            ident => Token::IDENT(ident),
        }
    }

    //TODO Seems vaguely Iterable?
    pub fn next_token(&mut self) -> Token<'a> {
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

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
        match next {
            Token::EOF => None,
            _ => Some(next),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let input = "1,2,3;";
        let expected = vec![
            Token::INT("1"),
            Token::COMMA,
            Token::INT("2"),
            Token::COMMA,
            Token::INT("3"),
            Token::SEMICOLON,
        ];
        let lex = Lexer::new(input);
        let result = lex.collect::<Vec<Token>>();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_next_token_int() {
        let input = "xyz";
        let expected = vec![Token::IDENT("xyz")];
        let mut lexer = super::Lexer::new(input);

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
        let mut lexer = super::Lexer::new(input);

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
        let mut lexer = super::Lexer::new(monkey_example);
        let expected = vec![
            Token::LET,
            Token::IDENT("five"),
            Token::ASSIGN,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten"),
            Token::ASSIGN,
            Token::INT("10"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add"),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x"),
            Token::COMMA,
            Token::IDENT("y"),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x"),
            Token::PLUS,
            Token::IDENT("y"),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result"),
            Token::ASSIGN,
            Token::IDENT("add"),
            Token::LPAREN,
            Token::IDENT("five"),
            Token::COMMA,
            Token::IDENT("ten"),
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
        let mut lexer = super::Lexer::new(monkey_example);
        let expected = vec![
            Token::LET,
            Token::IDENT("five"),
            Token::ASSIGN,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten"),
            Token::ASSIGN,
            Token::INT("10"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add"),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x"),
            Token::COMMA,
            Token::IDENT("y"),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x"),
            Token::PLUS,
            Token::IDENT("y"),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result"),
            Token::ASSIGN,
            Token::IDENT("add"),
            Token::LPAREN,
            Token::IDENT("five"),
            Token::COMMA,
            Token::IDENT("ten"),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::INT("5"),
            Token::LT,
            Token::INT("10"),
            Token::GT,
            Token::INT("5"),
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
        let mut lexer = super::Lexer::new(monkey_example);
        let expected = vec![
            Token::LET,
            Token::IDENT("five"),
            Token::ASSIGN,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten"),
            Token::ASSIGN,
            Token::INT("10"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add"),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x"),
            Token::COMMA,
            Token::IDENT("y"),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x"),
            Token::PLUS,
            Token::IDENT("y"),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result"),
            Token::ASSIGN,
            Token::IDENT("add"),
            Token::LPAREN,
            Token::IDENT("five"),
            Token::COMMA,
            Token::IDENT("ten"),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::INT("5"),
            Token::LT,
            Token::INT("10"),
            Token::GT,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5"),
            Token::LT,
            Token::INT("10"),
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
        let mut lexer = super::Lexer::new(monkey_example);
        let expected = vec![
            Token::LET,
            Token::IDENT("five"),
            Token::ASSIGN,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten"),
            Token::ASSIGN,
            Token::INT("10"),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add"),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x"),
            Token::COMMA,
            Token::IDENT("y"),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x"),
            Token::PLUS,
            Token::IDENT("y"),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result"),
            Token::ASSIGN,
            Token::IDENT("add"),
            Token::LPAREN,
            Token::IDENT("five"),
            Token::COMMA,
            Token::IDENT("ten"),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::INT("5"),
            Token::LT,
            Token::INT("10"),
            Token::GT,
            Token::INT("5"),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5"),
            Token::LT,
            Token::INT("10"),
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
            Token::INT("10"),
            Token::EQ,
            Token::INT("10"),
            Token::SEMICOLON,
            Token::INT("10"),
            Token::NOTEQ,
            Token::INT("9"),
            Token::SEMICOLON,
            Token::EOF,
        ];

        for (pos, token) in expected.iter().enumerate() {
            let next_token = lexer.next_token();
            assert_eq!(*token, next_token, "Failed at position: {}", pos);
        }
    }
}
