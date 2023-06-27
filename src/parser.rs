use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Option<Program> {
        let program = Program::new(Vec::new());

        Some(program)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_let_statements() {
        let input = "let x = 5; let y = 10; let foobar = 838383;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        if let Some(program) = program {
            if program.statements.len() != 3 {
                panic!(
                    "program.statements does not contain 3 statements. Got={}",
                    program.statements.len()
                );
            }
        } else {
            panic!("ParseProgram() returned nil")
        }
    }
}
