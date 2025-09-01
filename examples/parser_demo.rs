use oxide::{Expr, Stmt, parse_source};

fn main() {
    println!("Oxide Parser Demo");
    println!("================");

    // Example 1: Simple let statement
    println!("\n1. Simple let statement:");
    let source1 = "let x = 42;";
    println!("Source: {}", source1);
    match parse_source(source1) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 2: Arithmetic expressions with operator precedence
    println!("\n2. Arithmetic expressions:");
    let source2 = "let result = 1 + 2 * 3 - 4 / 2;";
    println!("Source: {}", source2);
    match parse_source(source2) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 3: Grouped expressions
    println!("\n3. Grouped expressions:");
    let source3 = "let value = (1 + 2) * (3 - 4);";
    println!("Source: {}", source3);
    match parse_source(source3) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 4: Unary expressions
    println!("\n4. Unary expressions:");
    let source4 = "let negative = -42; let double_negative = --10;";
    println!("Source: {}", source4);
    match parse_source(source4) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 5: Block statements
    println!("\n5. Block statements:");
    let source5 = r#"{
        let x = 5;
        let y = 10;
        x + y;
    }"#;
    println!("Source: {}", source5);
    match parse_source(source5) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 6: Multiple statements
    println!("\n6. Multiple statements:");
    let source6 = r#"
        let a = 1;
        let b = 2;
        let sum = a + b;
        sum * 2;
    "#;
    println!("Source: {}", source6);
    match parse_source(source6) {
        Ok(program) => {
            println!("Parsed AST:");
            for (i, stmt) in program.statements.iter().enumerate() {
                println!("  Statement {}: {}", i + 1, stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 7: Expression statement
    println!("\n7. Expression statements:");
    let source7 = "42; 3 + 4; (1 + 2) * 3;";
    println!("Source: {}", source7);
    match parse_source(source7) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 8: Error handling
    println!("\n8. Error handling:");
    let source8 = "let x = ; let y = 42"; // Missing expression and semicolon
    println!("Source: {}", source8);
    match parse_source(source8) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }

    // Example 9: Complex nested expression
    println!("\n9. Complex nested expressions:");
    let source9 = "let complex = ((1 + 2) * 3) - (4 / (5 + 6));";
    println!("Source: {}", source9);
    match parse_source(source9) {
        Ok(program) => {
            println!("Parsed AST:");
            for stmt in &program.statements {
                println!("  {}", stmt);
            }

            // Let's also show the detailed structure
            if let Some(Stmt::Let { name, value }) = program.statements.first() {
                println!("\nDetailed AST structure for variable '{}':", name);
                print_expr_structure(value, 0);
            }
        }
        Err(errors) => println!("Parse errors: {}", errors),
    }
}

fn print_expr_structure(expr: &Expr, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match expr {
        Expr::Number(n) => println!("{}Number({})", indent_str, n),
        Expr::Identifier(name) => println!("{}Identifier({})", indent_str, name),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            println!("{}Binary({:?}):", indent_str, operator);
            println!("{}  Left:", indent_str);
            print_expr_structure(left, indent + 2);
            println!("{}  Right:", indent_str);
            print_expr_structure(right, indent + 2);
        }
        Expr::Unary { operator, operand } => {
            println!("{}Unary({:?}):", indent_str, operator);
            println!("{}  Operand:", indent_str);
            print_expr_structure(operand, indent + 2);
        }
        Expr::Grouping(inner) => {
            println!("{}Grouping:", indent_str);
            print_expr_structure(inner, indent + 1);
        }
    }
}
