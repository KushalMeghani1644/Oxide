pub mod ast;
pub mod error;
pub mod parse;

pub use ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
pub use error::{ParseError, ParseErrors, ParseResult};
pub use parse::Parser;

// Convenience function to parse source code directly
pub fn parse_source(source: &str) -> Result<Program, ParseErrors> {
    let mut parser = Parser::from_source(source);
    parser.parse()
}

// Convenience function to parse tokens directly
pub fn parse_tokens(tokens: Vec<crate::lexer::Token>) -> Result<Program, ParseErrors> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
