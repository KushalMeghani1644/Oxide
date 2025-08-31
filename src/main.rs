mod lexer;

use crate::lexer::{Lexer, Token};

fn main() {
    // Test cases to demonstrate lexer improvements
    let test_inputs = vec![
        "let x = 5;",
        "let y = 10 + 20;",
        "let result = (x * y) / 2;",
        "let foo_bar = 123;",
        "   let   spaced   =   42   ;   ",
        "invalid@chars#here$",
    ];

    for (i, input) in test_inputs.iter().enumerate() {
        println!("=== Test Case {} ===", i + 1);
        println!("Input: {}", input);
        println!("Tokens:");

        let mut lexer = Lexer::new(input);

        // Method 1: Using next_token() in a loop
        loop {
            let token = lexer.next_token();
            println!("  {:?}", token);

            if token == Token::EOF {
                break;
            }
        }

        println!();
    }

    // Demonstrate iterator functionality
    println!("=== Iterator Demo ===");
    let input = "let sum = a + b;";
    println!("Input: {}", input);
    println!("Using iterator:");

    let lexer = Lexer::new(input);
    for token in lexer {
        println!("  {}", token);
    }

    println!();

    // Demonstrate tokenize() method
    println!("=== Tokenize Method Demo ===");
    let input = "let result = (10 - 5) * 2;";
    println!("Input: {}", input);

    let mut lexer = Lexer::new(input);
    let all_tokens = lexer.tokenize();

    println!("All tokens at once: {:?}", all_tokens);
    println!();

    // Demonstrate lexer state methods
    println!("=== Lexer State Demo ===");
    let input = "abc 123";
    println!("Input: {}", input);

    let mut lexer = Lexer::new(input);

    while !lexer.is_at_end() {
        let pos = lexer.position();
        let token = lexer.next_token();
        println!("  Position {}: {:?}", pos, token);

        if token == Token::EOF {
            break;
        }
    }
}
