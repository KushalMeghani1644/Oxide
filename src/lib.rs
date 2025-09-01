pub mod lexer;
pub mod parser;

pub use lexer::{LexError, Lexer, Token};
pub use parser::{
    BinaryOp, Expr, ParseError, ParseErrors, Parser, Program, Stmt, UnaryOp, parse_source,
    parse_tokens,
};

// Convenience function to parse source code in one step
pub fn compile(source: &str) -> Result<Program, Box<dyn std::error::Error>> {
    match parse_source(source) {
        Ok(program) => Ok(program),
        Err(errors) => Err(Box::new(errors)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        let source = "let x = 5 + 3 * 2; let y = -x;";
        let program = compile(source).unwrap();

        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_parse_source_convenience() {
        let source = "let hello = 42;";
        let program = parse_source(source).unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Let { name, value } => {
                assert_eq!(name, "hello");
                assert_eq!(*value, Expr::Number(42));
            }
            _ => panic!("Expected let statement"),
        }
    }
}
