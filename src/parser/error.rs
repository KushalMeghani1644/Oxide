use crate::lexer::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<String>,
        found: Token,
        position: usize,
    },
    UnexpectedEndOfInput {
        expected: Vec<String>,
    },
    InvalidExpression {
        message: String,
        position: usize,
    },
    InvalidStatement {
        message: String,
        position: usize,
    },
    MissingExpression {
        context: String,
        position: usize,
    },
    MissingSemicolon {
        position: usize,
    },
    InvalidOperator {
        operator: Token,
        position: usize,
    },
}

impl ParseError {
    pub fn unexpected_token(expected: Vec<&str>, found: Token, position: usize) -> Self {
        ParseError::UnexpectedToken {
            expected: expected.into_iter().map(|s| s.to_string()).collect(),
            found,
            position,
        }
    }

    pub fn unexpected_eof(expected: Vec<&str>) -> Self {
        ParseError::UnexpectedEndOfInput {
            expected: expected.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn invalid_expression(message: &str, position: usize) -> Self {
        ParseError::InvalidExpression {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_statement(message: &str, position: usize) -> Self {
        ParseError::InvalidStatement {
            message: message.to_string(),
            position,
        }
    }

    pub fn missing_expression(context: &str, position: usize) -> Self {
        ParseError::MissingExpression {
            context: context.to_string(),
            position,
        }
    }

    pub fn missing_semicolon(position: usize) -> Self {
        ParseError::MissingSemicolon { position }
    }

    pub fn invalid_operator(operator: Token, position: usize) -> Self {
        ParseError::InvalidOperator { operator, position }
    }

    pub fn position(&self) -> Option<usize> {
        match self {
            ParseError::UnexpectedToken { position, .. } => Some(*position),
            ParseError::InvalidExpression { position, .. } => Some(*position),
            ParseError::InvalidStatement { position, .. } => Some(*position),
            ParseError::MissingExpression { position, .. } => Some(*position),
            ParseError::MissingSemicolon { position } => Some(*position),
            ParseError::InvalidOperator { position, .. } => Some(*position),
            ParseError::UnexpectedEndOfInput { .. } => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                position,
            } => {
                if expected.len() == 1 {
                    write!(
                        f,
                        "Parse error at position {}: expected '{}', found '{}'",
                        position, expected[0], found
                    )
                } else if expected.len() == 2 {
                    write!(
                        f,
                        "Parse error at position {}: expected '{}' or '{}', found '{}'",
                        position, expected[0], expected[1], found
                    )
                } else {
                    write!(
                        f,
                        "Parse error at position {}: expected one of [{}], found '{}'",
                        position,
                        expected.join(", "),
                        found
                    )
                }
            }
            ParseError::UnexpectedEndOfInput { expected } => {
                if expected.len() == 1 {
                    write!(
                        f,
                        "Parse error: unexpected end of input, expected '{}'",
                        expected[0]
                    )
                } else {
                    write!(
                        f,
                        "Parse error: unexpected end of input, expected one of [{}]",
                        expected.join(", ")
                    )
                }
            }
            ParseError::InvalidExpression { message, position } => {
                write!(f, "Parse error at position {}: {}", position, message)
            }
            ParseError::InvalidStatement { message, position } => {
                write!(f, "Parse error at position {}: {}", position, message)
            }
            ParseError::MissingExpression { context, position } => {
                write!(
                    f,
                    "Parse error at position {}: missing expression in {}",
                    position, context
                )
            }
            ParseError::MissingSemicolon { position } => {
                write!(f, "Parse error at position {}: missing semicolon", position)
            }
            ParseError::InvalidOperator { operator, position } => {
                write!(
                    f,
                    "Parse error at position {}: invalid operator '{}'",
                    position, operator
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseErrors {
    pub errors: Vec<ParseError>,
}

impl ParseErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn first(&self) -> Option<&ParseError> {
        self.errors.first()
    }
}

impl Default for ParseErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ParseErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            write!(f, "No parse errors")
        } else if self.errors.len() == 1 {
            write!(f, "{}", self.errors[0])
        } else {
            writeln!(f, "Parse errors:")?;
            for (i, error) in self.errors.iter().enumerate() {
                writeln!(f, "  {}: {}", i + 1, error)?;
            }
            Ok(())
        }
    }
}

impl std::error::Error for ParseErrors {}

impl From<ParseError> for ParseErrors {
    fn from(error: ParseError) -> Self {
        let mut errors = ParseErrors::new();
        errors.add(error);
        errors
    }
}
