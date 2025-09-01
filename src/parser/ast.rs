use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Expression(Expr),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, stmt: Stmt) {
        self.statements.push(stmt);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

// Helper methods for AST construction
impl Expr {
    pub fn number(value: i64) -> Self {
        Expr::Number(value)
    }

    pub fn identifier(name: String) -> Self {
        Expr::Identifier(name)
    }

    pub fn binary(left: Expr, operator: BinaryOp, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: UnaryOp, operand: Expr) -> Self {
        Expr::Unary {
            operator,
            operand: Box::new(operand),
        }
    }

    pub fn grouping(expr: Expr) -> Self {
        Expr::Grouping(Box::new(expr))
    }
}

impl BinaryOp {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Subtract),
            Token::Multiply => Some(BinaryOp::Multiply),
            Token::Divide => Some(BinaryOp::Divide),
            _ => None,
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Subtract => 1,
            BinaryOp::Multiply | BinaryOp::Divide => 2,
        }
    }
}

impl UnaryOp {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Minus => Some(UnaryOp::Negate),
            _ => None,
        }
    }
}

impl Stmt {
    pub fn let_statement(name: String, value: Expr) -> Self {
        Stmt::Let { name, value }
    }

    pub fn expression(expr: Expr) -> Self {
        Stmt::Expression(expr)
    }

    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block(statements)
    }
}

// Display implementations for pretty printing
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expr::Unary { operator, operand } => {
                write!(f, "({}{})", operator, operand)
            }
            Expr::Grouping(expr) => write!(f, "({})", expr),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
        }
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Let { name, value } => write!(f, "let {} = {};", name, value),
            Stmt::Expression(expr) => write!(f, "{};", expr),
            Stmt::Block(statements) => {
                writeln!(f, "{{")?;
                for stmt in statements {
                    writeln!(f, "  {}", stmt)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}
