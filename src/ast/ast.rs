use crate::parser::token::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self) -> dyn Node;
}

pub trait Expression: Node {
    fn expression_node(&self) -> dyn Node;
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Program {
        Program { statements }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl LetStatement { 
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement {
        LetStatement { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        todo!()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) -> impl Node {
        self //?
    }

}

pub struct Identifier {
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        todo!()
    }
    
}

impl Expression for Identifier {
    fn expression_node(&self) -> impl Node {
    }
}