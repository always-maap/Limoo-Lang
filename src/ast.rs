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

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    Lit(Literal),
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    While(Box<Expression>, BlockStatement),
    Function(Vec<String>, BlockStatement),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Assign(String, Token, Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Ident(id) => write!(f, "{}", id),
            Expression::Lit(lit) => write!(f, "{}", lit),
            Expression::Prefix(prefix, expr) => write!(f, "({}{})", prefix, expr),
            Expression::Infix(left_expression, operator, right_expression) => {
                write!(f, "({} {} {})", left_expression, operator, right_expression)
            }
            Expression::If(condition, then_block, else_block) => {
                if let Some(else_block) = else_block {
                    write!(
                        f,
                        "if {} {{ {} }} else {{ {} }}",
                        condition,
                        format_statements(then_block),
                        format_statements(else_block)
                    )
                } else {
                    write!(f, "if {} {{ {} }}", condition, format_statements(then_block))
                }
            }
            Expression::While(condition, block) => {
                write!(f, "while {} {{ {} }}", condition, format_statements(block))
            }
            Expression::Function(params, _block) => {
                write!(f, "fn({}) {{...}}", params.join(", "),)
            }
            Expression::FunctionCall(function_expression, args) => {
                write!(f, "{}({})", function_expression, format_expressions(args))
            }
            Expression::Assign(left_expression, operator, right_expression) => {
                write!(f, "{} {} {}", left_expression, operator, right_expression)
            }
            Expression::Index(left_expression, right_expression) => {
                write!(f, "({}[{}])", left_expression, right_expression)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::String(s) => write!(f, r#""{}""#, s),
            Literal::Array(arr) => write!(f, "[{}]", format_expressions(arr)),
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

fn format_expressions(stmts: &[Expression]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
