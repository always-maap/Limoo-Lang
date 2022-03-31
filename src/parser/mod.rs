use crate::{
    ast::{BlockStatement, Expression, Literal, Statement},
    lexer::Lexer,
    token::Token,
};

use self::{
    error::{ParserError, ParserErrors},
    precedence::{token_to_precedence, Precedence},
};

mod error;
pub mod parser_test;
mod precedence;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let errors = vec![];

        Parser {
            lexer,
            current_token,
            peek_token,
            errors,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<Vec<Statement>, ParserErrors> {
        let mut program = Vec::new();

        while !self.current_token_is(&Token::EOF) {
            match self.parse_statement() {
                Ok(statement) => program.push(statement),
                Err(e) => self.errors.push(e),
            }
            self.next_token();
        }

        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier = match &self.peek_token {
            Token::IDENT(ref id) => id.clone(),
            token => {
                return Err(self.error_no_identifier(token));
            }
        };
        self.next_token();

        self.expect_peek(&Token::ASSIGN)?;
        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::Let(identifier, expression))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Statement::Expr(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut left_expression = match self.current_token {
            Token::IDENT(ref id) => Ok(Expression::Ident(id.clone())),
            Token::INT(value) => Ok(Expression::Lit(Literal::Integer(value))),
            Token::BOOLEAN(boolean) => Ok(Expression::Lit(Literal::Boolean(boolean))),
            Token::STRING(ref string) => Ok(Expression::Lit(Literal::String(string.clone()))),
            Token::BANG | Token::MINUS => self.parse_prefix_expression(),
            Token::LPAREN => self.parse_group_expression(),
            Token::IF => self.parse_if_expression(),
            Token::FUNCTION => self.parse_fn_expressions(),
            _ => {
                return Err(ParserError::new(format!(
                    "no prefix parse function for {:?}",
                    self.current_token
                )))
            }
        };

        while !self.peek_token_is(&Token::SEMICOLON) && precedence < token_to_precedence(&self.peek_token) {
            match self.peek_token {
                Token::GT
                | Token::LT
                | Token::EQ
                | Token::NOT_EQ
                | Token::PLUS
                | Token::MINUS
                | Token::ASTERISK
                | Token::SLASH => {
                    self.next_token();
                    let expression = left_expression.unwrap();
                    left_expression = self.parse_infix_expression(expression)
                }
                Token::LPAREN => {
                    self.next_token();
                    let expression = left_expression.unwrap();
                    left_expression = self.parse_call_expression(expression)
                }
                _ => return left_expression,
            }
        }

        left_expression
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.current_token.clone();
        self.next_token();

        let right_expression = self.parse_expression(Precedence::PREFIX)?;

        Ok(Expression::Prefix(operator, Box::new(right_expression)))
    }

    fn parse_infix_expression(&mut self, left_expression: Expression) -> Result<Expression, ParserError> {
        let infix_operator = self.current_token.clone();

        let precedence = token_to_precedence(&infix_operator);
        self.next_token();

        let right_expression = self.parse_expression(precedence)?;

        Ok(Expression::Infix(
            Box::new(left_expression),
            infix_operator,
            Box::new(right_expression),
        ))
    }

    fn parse_group_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST)?;
        self.expect_peek(&Token::RPAREN)?;
        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_peek(&Token::LPAREN)?;
        self.next_token();

        let condition = self.parse_expression(Precedence::LOWEST)?;

        self.expect_peek(&Token::RPAREN)?;
        self.expect_peek(&Token::LBRACE)?;

        let consequence = self.parse_block_statement()?;

        let alternative = if self.peek_token_is(&Token::ELSE) {
            self.next_token();
            self.expect_peek(&Token::LBRACE)?;
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Ok(Expression::If(Box::new(condition), consequence, alternative))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        self.next_token();

        let mut statements = Vec::new();

        while !self.current_token_is(&Token::RBRACE) && !self.current_token_is(&Token::EOF) {
            if let Ok(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        Ok(statements)
    }

    fn parse_fn_expressions(&mut self) -> Result<Expression, ParserError> {
        self.expect_peek(&Token::LPAREN)?;

        let parameters = self.parse_fn_parameters()?;

        self.expect_peek(&Token::LBRACE)?;

        let body = self.parse_block_statement()?;

        Ok(Expression::Function(parameters, body))
    }

    fn parse_fn_parameters(&mut self) -> Result<Vec<String>, ParserError> {
        let mut parameters = Vec::new();

        if self.peek_token_is(&Token::RPAREN) {
            self.next_token();
            return Ok(parameters);
        }

        self.next_token();

        match &self.current_token {
            Token::IDENT(ref id) => parameters.push(id.clone()),
            token => return Err(self.error_no_identifier(token)),
        }

        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();

            match &self.current_token {
                Token::IDENT(ref id) => parameters.push(id.clone()),
                token => return Err(self.error_no_identifier(token)),
            }
        }

        self.expect_peek(&Token::RPAREN)?;

        Ok(parameters)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParserError> {
        let arguments = self.parse_call_arguments()?;

        Ok(Expression::FunctionCall(Box::new(function), arguments))
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut arguments = Vec::new();

        self.next_token();

        if self.current_token_is(&Token::RPAREN) {
            self.next_token();
            return Ok(arguments);
        }

        arguments.push(self.parse_expression(Precedence::LOWEST)?);

        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();

            arguments.push(self.parse_expression(Precedence::LOWEST)?);
        }

        self.expect_peek(&Token::RPAREN)?;

        Ok(arguments)
    }

    fn error_no_identifier(&self, token: &Token) -> ParserError {
        ParserError::new(format!("Expected an identifier but got {:?}", token.clone()))
    }

    fn current_token_is(&self, token: &Token) -> bool {
        self.current_token == *token
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        self.peek_token == *token
    }

    fn expect_peek(&mut self, token: &Token) -> Result<(), ParserError> {
        if self.peek_token_is(token) {
            self.next_token();
            Ok(())
        } else {
            Err(ParserError::new(format!(
                "Expected {:?}, got {:?}",
                token, self.peek_token
            )))
        }
    }
}
