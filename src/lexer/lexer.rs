use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(i64),
    Ident(String),

    // Keywords
    Let,

    // Operators
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,

    // Delimiters
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Special
    EOF,
    Illegal(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Let => write!(f, "let"),
            Token::Equals => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Semicolon => write!(f, ";"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::EOF => write!(f, "EOF"),
            Token::Illegal(c) => write!(f, "ILLEGAL({})", c),
        }
    }
}

#[derive(Debug)]
pub enum LexError {
    InvalidNumber(String),
    UnterminatedString,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            LexError::UnterminatedString => write!(f, "Unterminated string literal"),
        }
    }
}

impl std::error::Error for LexError {}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();

        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }

    /// Returns the current character without advancing the position
    fn peek(&self) -> Option<char> {
        self.current_char
    }

    /// Returns the character at the given offset from current position
    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    /// Advances to the next character and returns the previous one
    fn advance(&mut self) -> Option<char> {
        let current = self.current_char;
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
        current
    }

    /// Skips characters while the condition is true
    fn skip_while<F>(&mut self, condition: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if condition(ch) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Collects characters while the condition is true
    fn collect_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if condition(ch) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    /// Skips whitespace characters
    fn skip_whitespace(&mut self) {
        self.skip_while(|ch| ch.is_whitespace());
    }

    /// Reads a number token
    fn read_number(&mut self) -> Result<Token, LexError> {
        let number_str = self.collect_while(|ch| ch.is_ascii_digit());

        match number_str.parse::<i64>() {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(LexError::InvalidNumber(number_str)),
        }
    }

    /// Reads an identifier or keyword
    fn read_identifier(&mut self) -> Token {
        let ident = self.collect_while(|ch| ch.is_alphanumeric() || ch == '_');

        match ident.as_str() {
            "let" => Token::Let,
            _ => Token::Ident(ident),
        }
    }

    /// Gets the next token from the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek() {
            None => Token::EOF,
            Some(ch) => match ch {
                '=' => {
                    self.advance();
                    Token::Equals
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Multiply
                }
                '/' => {
                    self.advance();
                    Token::Divide
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                '(' => {
                    self.advance();
                    Token::LeftParen
                }
                ')' => {
                    self.advance();
                    Token::RightParen
                }
                '{' => {
                    self.advance();
                    Token::LeftBrace
                }
                '}' => {
                    self.advance();
                    Token::RightBrace
                }
                '0'..='9' => match self.read_number() {
                    Ok(token) => token,
                    Err(_) => {
                        self.advance();
                        Token::Illegal(ch)
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                _ => {
                    self.advance();
                    Token::Illegal(ch)
                }
            },
        }
    }

    /// Tokenizes the entire input and returns a vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = token == Token::EOF;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        tokens
    }

    /// Returns the current position in the input
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns true if we've reached the end of input
    pub fn is_at_end(&self) -> bool {
        self.current_char.is_none()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::EOF {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("=+(){}*;");

        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Multiply);
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("123 456");

        assert_eq!(lexer.next_token(), Token::Number(123));
        assert_eq!(lexer.next_token(), Token::Number(456));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let mut lexer = Lexer::new("let x foo_bar");

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Ident("foo_bar".to_string()));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_complete_statement() {
        let mut lexer = Lexer::new("let x = 42;");

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Number(42));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("  let   x   =   42  ;  ");

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Number(42));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_iterator_implementation() {
        let lexer = Lexer::new("let x = 5;");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("x".to_string()),
                Token::Equals,
                Token::Number(5),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_tokenize_method() {
        let mut lexer = Lexer::new("let x = 5;");
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("x".to_string()),
                Token::Equals,
                Token::Number(5),
                Token::Semicolon,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_illegal_characters() {
        let mut lexer = Lexer::new("@#$");

        assert_eq!(lexer.next_token(), Token::Illegal('@'));
        assert_eq!(lexer.next_token(), Token::Illegal('#'));
        assert_eq!(lexer.next_token(), Token::Illegal('$'));
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
