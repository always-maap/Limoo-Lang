use crate::{
    ast::{Expression, Statement},
    lexer::Lexer,
    token::Token,
};

use self::{
    error::{ParserError, ParserErrors},
    precedence::Precedence,
};

mod error;
#[cfg(test)]
mod parser_test;
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

        while self.current_token_is(&Token::EOF) {
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

        self.expect_peek(&Token::SEMICOLON)?;

        Ok(Statement::Expr(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        unimplemented!()
    }

    fn error_no_identifier(&self, token: &Token) -> ParserError {
        ParserError::new(format!(
            "Expected an identifier but got {:?}",
            token.clone()
        ))
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
