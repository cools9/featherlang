use crate::ast::ast::{Ast, Operator};
use crate::lexer::lexer::TokenTypes;

pub struct Parser {
    tokens: Vec<TokenTypes>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenTypes>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        let ast = self.statement()?;

        if !self.is_at_end() {
            return Err(format!("Unexpected token: {:?}", self.peek()));
        }

        Ok(ast)
    }

    fn statement(&mut self) -> Result<Ast, String> {
        if self.matches(&TokenTypes::LET) {
            let name = match self.advance() {
                Some(TokenTypes::IDENTIFIER(name)) => name,
                other => {
                    return Err(format!("Expected variable name, got {:?}", other));
                }
            };

            self.consume(TokenTypes::Equal, "Expected '=' after variable name")?;

            let value = self.expression()?;
            self.matches(&TokenTypes::Semicolon);

            return Ok(Ast::Let {
                name,
                value: Box::new(value),
            });
        }

        if self.matches(&TokenTypes::PRINT) {
            let value = self.expression()?;
            self.matches(&TokenTypes::Semicolon);

            return Ok(Ast::Print {
                value: Box::new(value),
            });
        }

        let expr = self.expression()?;
        self.matches(&TokenTypes::Semicolon);
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Ast, String> {
        self.addition()
    }

    fn addition(&mut self) -> Result<Ast, String> {
        let mut expr = self.multiplication()?;

        loop {
            let operator = if self.matches(&TokenTypes::Plus) {
                Some(Operator::Add)
            } else if self.matches(&TokenTypes::Minus) {
                Some(Operator::Subtract)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };

            let right = self.multiplication()?;

            expr = Ast::BinaryExpressions {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Ast, String> {
        let mut expr = self.unary()?;

        loop {
            let operator = if self.matches(&TokenTypes::Star) {
                Some(Operator::Multiply)
            } else if self.matches(&TokenTypes::SLASH) {
                Some(Operator::Divide)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };

            let right = self.unary()?;

            expr = Ast::BinaryExpressions {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Ast, String> {
        if self.matches(&TokenTypes::Minus) {
            let operand = self.unary()?;

            return Ok(Ast::UnaryExpression {
                operator: Operator::Subtract,
                operand: Box::new(operand),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Ast, String> {
        match self.advance() {
            Some(TokenTypes::NUMBER(value)) => Ok(Ast::Number { value }),

            Some(TokenTypes::STRING(value)) => Ok(Ast::String { value }),

            Some(TokenTypes::TRUE) => Ok(Ast::Bool { value: true }),

            Some(TokenTypes::FALSE) => Ok(Ast::Bool { value: false }),

            Some(TokenTypes::LeftParen) => {
                let expr = self.expression()?;

                self.consume(TokenTypes::RightParen, "Expected ')' after expression")?;

                Ok(expr)
            }

            other => Err(format!("Expected an expression, got {:?}", other)),
        }
    }

    fn matches(&mut self, expected: &TokenTypes) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &TokenTypes) -> bool {
        match (self.peek(), expected) {
            (Some(TokenTypes::LET), TokenTypes::LET)
            | (Some(TokenTypes::PRINT), TokenTypes::PRINT)
            | (Some(TokenTypes::Plus), TokenTypes::Plus)
            | (Some(TokenTypes::Minus), TokenTypes::Minus)
            | (Some(TokenTypes::Star), TokenTypes::Star)
            | (Some(TokenTypes::SLASH), TokenTypes::SLASH)
            | (Some(TokenTypes::Equal), TokenTypes::Equal)
            | (Some(TokenTypes::Semicolon), TokenTypes::Semicolon)
            | (Some(TokenTypes::LeftParen), TokenTypes::LeftParen)
            | (Some(TokenTypes::RightParen), TokenTypes::RightParen) => true,
            _ => false,
        }
    }

    fn consume(&mut self, expected: TokenTypes, message: &str) -> Result<(), String> {
        if self.matches(&expected) {
            Ok(())
        } else {
            Err(message.to_string())
        }
    }

    fn advance(&mut self) -> Option<TokenTypes> {
        if self.is_at_end() {
            return None;
        }

        let token = self.tokens[self.current].clone();
        self.current += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&TokenTypes> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
