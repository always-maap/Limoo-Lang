use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub enum Node {
    Program(Vec<Statement>),
    Stmt(Statement),
    Expr(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Program(stmts) => write!(f, "{}", format_statements(stmts)),
            Node::Stmt(stmt) => write!(f, "{}", stmt),
            Node::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expr(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(id, expr) => write!(f, "let {} = {};", id, expr),
            Statement::Return(expr) => write!(f, "return {};", expr),
            Statement::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    Lit(Literal),
    Prefix(Token, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Ident(id) => write!(f, "{}", id),
            Expression::Lit(lit) => write!(f, "{}", lit),
            Expression::Prefix(prefix, expr) => write!(f, "({}{})", prefix, expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{}", i),
        }
    }
}

fn format_statements(stmts: &[Statement]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join("")
}
