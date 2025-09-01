use oxide::{parse_source, Expr, Stmt};
use std::io::{self, Write};

fn main() {
    println!("Oxide Language REPL");
    println!("Type 'help' for commands, 'quit' to exit");
    println!("Enter Oxide code to parse and see the AST\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" | "h" => {
                        print_help();
                        continue;
                    }
                    "clear" | "cls" => {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    _ => {
                        handle_input(input);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}

fn print_help() {
    println!("Commands:");
    println!("  help, h     - Show this help message");
    println!("  quit, exit, q - Exit the REPL");
    println!("  clear, cls  - Clear the screen");
    println!("\nExamples:");
    println!("  let x = 42;");
    println!("  1 + 2 * 3;");
    println!("  (1 + 2) * (3 - 4);");
    println!("  -42;");
    println!("  {{ let x = 5; x + 10; }}");
    println!();
}

fn handle_input(input: &str) {
    match parse_source(input) {
        Ok(program) => {
            if program.statements.is_empty() {
                println!("No statements parsed");
                return;
            }

            println!("✓ Parsed successfully!");
            println!("AST:");

            for (i, stmt) in program.statements.iter().enumerate() {
                if program.statements.len() > 1 {
                    println!("  Statement {}:", i + 1);
                }
                print_statement(stmt, if program.statements.len() > 1 { 2 } else { 1 });
            }
            println!();
        }
        Err(errors) => {
            println!("✗ Parse failed:");
            for (i, error) in errors.errors.iter().enumerate() {
                if errors.errors.len() > 1 {
                    println!("  Error {}: {}", i + 1, error);
                } else {
                    println!("  {}", error);
                }
            }
            println!();
        }
    }
}

fn print_statement(stmt: &Stmt, indent_level: usize) {
    let indent = "  ".repeat(indent_level);

    match stmt {
        Stmt::Let { name, value } => {
            println!("{}Let Statement:", indent);
            println!("{}  Variable: {}", indent, name);
            println!("{}  Value:", indent);
            print_expression(value, indent_level + 2);
        }
        Stmt::Expression(expr) => {
            println!("{}Expression Statement:", indent);
            print_expression(expr, indent_level + 1);
        }
        Stmt::Block(statements) => {
            println!("{}Block Statement:", indent);
            println!("{}  Statements ({}):", indent, statements.len());
            for (i, stmt) in statements.iter().enumerate() {
                println!("{}    [{}]:", indent, i);
                print_statement(stmt, indent_level + 3);
            }
        }
    }
}

fn print_expression(expr: &Expr, indent_level: usize) {
    let indent = "  ".repeat(indent_level);

    match expr {
        Expr::Number(n) => {
            println!("{}Number: {}", indent, n);
        }
        Expr::Identifier(name) => {
            println!("{}Identifier: {}", indent, name);
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            println!("{}Binary Expression ({:?}):", indent, operator);
            println!("{}  Left:", indent);
            print_expression(left, indent_level + 2);
            println!("{}  Right:", indent);
            print_expression(right, indent_level + 2);
        }
        Expr::Unary { operator, operand } => {
            println!("{}Unary Expression ({:?}):", indent, operator);
            println!("{}  Operand:", indent);
            print_expression(operand, indent_level + 2);
        }
        Expr::Grouping(inner) => {
            println!("{}Grouped Expression:", indent);
            print_expression(inner, indent_level + 1);
        }
    }
}
