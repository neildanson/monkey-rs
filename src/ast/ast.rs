use crate::parser::token::token::Token;

pub struct Identifier {
    token: Token,
    value: String,
}

pub enum Expression {
    Identifier(Identifier),
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.token.literal.clone(),
            _ => "".to_string(),
        }
    }
}

pub enum Statement {
    LetStatement {
        token: Token,
        name: Identifier,
        value: Expression,
    },
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement { token, .. } => token.literal.clone(),
            _ => "".to_string(),
        }
    }
}

pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

impl Node {
    pub fn token_literal(&self) -> String {
        match self {
            Node::Expression(expression) => expression.token_literal(),
            Node::Statement(statement) => statement.token_literal(),
        }
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program { statements }
    }

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}
