use super::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use super::error::{ParseError, ParseErrors, ParseResult};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn from_source(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        Self::new(tokens)
    }

    /// Returns the current token without advancing
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    /// Returns the token at the given offset from current position
    fn peek_ahead(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.current + offset)
            .unwrap_or(&Token::EOF)
    }

    /// Returns the previous token
    fn previous(&self) -> &Token {
        if self.current > 0 {
            &self.tokens[self.current - 1]
        } else {
            &Token::EOF
        }
    }

    /// Advances to the next token and returns the previous one
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Checks if we're at the end of tokens
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }

    /// Checks if the current token matches any of the given tokens
    fn matches(&self, tokens: &[Token]) -> bool {
        for token in tokens {
            if std::mem::discriminant(self.peek()) == std::mem::discriminant(token) {
                return true;
            }
        }
        false
    }

    /// Consumes the current token if it matches the expected token
    fn consume(&mut self, expected: Token, message: &str) -> ParseResult<&Token> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::unexpected_token(
                vec![&format!("{}", expected)],
                self.peek().clone(),
                self.current,
            ))
        }
    }

    /// Synchronizes the parser after an error by finding the next statement boundary
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if matches!(self.previous(), Token::Semicolon) {
                return;
            }

            match self.peek() {
                Token::Let => return,
                Token::LeftBrace => return,
                _ => {}
            }

            self.advance();
        }
    }

    /// Parses a complete program
    pub fn parse(&mut self) -> Result<Program, ParseErrors> {
        let mut program = Program::new();
        let mut errors = ParseErrors::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => program.add_statement(stmt),
                Err(error) => {
                    errors.add(error);
                    self.synchronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }

    /// Parses a statement
    fn statement(&mut self) -> ParseResult<Stmt> {
        match self.peek() {
            Token::Let => self.let_statement(),
            Token::LeftBrace => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    /// Parses a let statement: let identifier = expression;
    fn let_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(Token::Let, "Expected 'let'")?;

        let name = match self.advance() {
            Token::Ident(name) => name.clone(),
            token => {
                return Err(ParseError::unexpected_token(
                    vec!["identifier"],
                    token.clone(),
                    self.current - 1,
                ));
            }
        };

        self.consume(Token::Equals, "Expected '=' after variable name")?;

        let value = self.expression()?;

        self.consume(Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Stmt::let_statement(name, value))
    }

    /// Parses a block statement: { statements... }
    fn block_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(Token::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();

        while !matches!(self.peek(), Token::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after block")?;

        Ok(Stmt::block(statements))
    }

    /// Parses an expression statement: expression;
    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(Token::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::expression(expr))
    }

    /// Parses an expression using precedence climbing
    fn expression(&mut self) -> ParseResult<Expr> {
        self.binary_expression(0)
    }

    /// Parses binary expressions with operator precedence
    fn binary_expression(&mut self, min_precedence: u8) -> ParseResult<Expr> {
        let mut left = self.unary_expression()?;

        while let Some(op) = BinaryOp::from_token(self.peek()) {
            if op.precedence() < min_precedence {
                break;
            }

            self.advance(); // consume operator
            let right = self.binary_expression(op.precedence() + 1)?;
            left = Expr::binary(left, op, right);
        }

        Ok(left)
    }

    /// Parses unary expressions: -expression
    fn unary_expression(&mut self) -> ParseResult<Expr> {
        if let Some(op) = UnaryOp::from_token(self.peek()) {
            self.advance(); // consume operator
            let operand = self.unary_expression()?;
            Ok(Expr::unary(op, operand))
        } else {
            self.primary_expression()
        }
    }

    /// Parses primary expressions: numbers, identifiers, grouped expressions
    fn primary_expression(&mut self) -> ParseResult<Expr> {
        match self.advance().clone() {
            Token::Number(value) => Ok(Expr::number(value)),
            Token::Ident(name) => Ok(Expr::identifier(name)),
            Token::LeftParen => {
                let expr = self.expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                Ok(Expr::grouping(expr))
            }
            token => Err(ParseError::unexpected_token(
                vec!["number", "identifier", "'('"],
                token,
                self.current - 1,
            )),
        }
    }

    /// Returns the current position
    pub fn position(&self) -> usize {
        self.current
    }

    /// Resets the parser to the beginning
    pub fn reset(&mut self) {
        self.current = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_simple_let_statement() {
        let mut parser = Parser::from_source("let x = 42;");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Let { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(*value, Expr::number(42));
            }
            _ => panic!("Expected let statement"),
        }
    }

    #[test]
    fn test_expression_statement() {
        let mut parser = Parser::from_source("42;");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expression(expr) => {
                assert_eq!(*expr, Expr::number(42));
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_binary_expression() {
        let mut parser = Parser::from_source("1 + 2 * 3;");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expression(expr) => match expr {
                Expr::Binary {
                    left,
                    operator,
                    right,
                } => {
                    assert_eq!(**left, Expr::number(1));
                    assert_eq!(*operator, BinaryOp::Add);
                    match right.as_ref() {
                        Expr::Binary {
                            left,
                            operator,
                            right,
                        } => {
                            assert_eq!(**left, Expr::number(2));
                            assert_eq!(*operator, BinaryOp::Multiply);
                            assert_eq!(**right, Expr::number(3));
                        }
                        _ => panic!("Expected binary expression for right operand"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_grouped_expression() {
        let mut parser = Parser::from_source("(1 + 2) * 3;");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expression(expr) => match expr {
                Expr::Binary {
                    left,
                    operator,
                    right,
                } => {
                    assert_eq!(*operator, BinaryOp::Multiply);
                    assert_eq!(**right, Expr::number(3));
                    match left.as_ref() {
                        Expr::Grouping(inner) => match inner.as_ref() {
                            Expr::Binary {
                                left,
                                operator,
                                right,
                            } => {
                                assert_eq!(**left, Expr::number(1));
                                assert_eq!(*operator, BinaryOp::Add);
                                assert_eq!(**right, Expr::number(2));
                            }
                            _ => panic!("Expected binary expression inside grouping"),
                        },
                        _ => panic!("Expected grouping expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_unary_expression() {
        let mut parser = Parser::from_source("-42;");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expression(expr) => match expr {
                Expr::Unary { operator, operand } => {
                    assert_eq!(*operator, UnaryOp::Negate);
                    assert_eq!(**operand, Expr::number(42));
                }
                _ => panic!("Expected unary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_block_statement() {
        let mut parser = Parser::from_source("{ let x = 5; 42; }");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Block(statements) => {
                assert_eq!(statements.len(), 2);
                match &statements[0] {
                    Stmt::Let { name, value } => {
                        assert_eq!(name, "x");
                        assert_eq!(*value, Expr::number(5));
                    }
                    _ => panic!("Expected let statement"),
                }
                match &statements[1] {
                    Stmt::Expression(expr) => {
                        assert_eq!(*expr, Expr::number(42));
                    }
                    _ => panic!("Expected expression statement"),
                }
            }
            _ => panic!("Expected block statement"),
        }
    }

    #[test]
    fn test_multiple_statements() {
        let mut parser = Parser::from_source("let x = 5; let y = 10; x + y;");
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_parse_error() {
        let mut parser = Parser::from_source("let x = ;");
        let result = parser.parse();

        assert!(result.is_err());
    }

    #[test]
    fn test_operator_precedence() {
        let mut parser = Parser::from_source("2 + 3 * 4;");
        let program = parser.parse().unwrap();

        // Should parse as 2 + (3 * 4), not (2 + 3) * 4
        match &program.statements[0] {
            Stmt::Expression(Expr::Binary {
                left,
                operator,
                right,
            }) => {
                assert_eq!(**left, Expr::number(2));
                assert_eq!(*operator, BinaryOp::Add);
                match right.as_ref() {
                    Expr::Binary {
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(**left, Expr::number(3));
                        assert_eq!(*operator, BinaryOp::Multiply);
                        assert_eq!(**right, Expr::number(4));
                    }
                    _ => panic!("Expected multiplication to have higher precedence"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }
}
