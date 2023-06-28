use crate::token::Token;

pub struct Identifier<'a> {
    token: Token<'a>,
    value: &'a str,
}

pub enum Expression<'a> {
    Identifier(Identifier<'a>),
}

impl<'a> Expression<'a> {
    pub fn token_literal(&self) -> &'a str {
        match self {
            Expression::Identifier(identifier) => identifier.token.identifier().clone(),
        }
    }
}

pub enum Statement<'a> {
    LetStatement {
        token: Token<'a>,
        name: Identifier<'a>,
        value: Expression<'a>,
    },
}

impl<'a> Statement<'a> {
    pub fn token_literal(&self) -> &'a str {
        match self {
            Statement::LetStatement { token, .. } => token.identifier().clone(),
        }
    }
}

pub enum Node<'a> {
    Expression(Expression<'a>),
    Statement(Statement<'a>),
}

impl<'a> Node<'a> {
    pub fn token_literal(&self) -> &'a str {
        match self {
            Node::Expression(expression) => expression.token_literal(),
            Node::Statement(statement) => statement.token_literal(),
        }
    }
}

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program { statements }
    }

    pub fn token_literal(&self) -> &'a str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}
