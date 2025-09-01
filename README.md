# Oxide Programming Language

A modern programming language implementation in Rust, featuring a complete lexer and recursive descent parser.

## Overview

Oxide is a simple programming language that supports:
- Variable declarations with `let`
- Arithmetic expressions with operator precedence
- Unary expressions (negation)
- Grouped expressions with parentheses
- Block statements with braces
- Expression statements

## Language Features

### Variables
```oxide
let x = 42;
let name = identifier;
```

### Arithmetic Expressions
```oxide
let sum = 1 + 2;
let product = 3 * 4;
let complex = (1 + 2) * 3 - 4 / 2;
```

### Unary Expressions
```oxide
let negative = -42;
let double_neg = --x;
```

### Block Statements
```oxide
{
    let x = 5;
    let y = 10;
    x + y;
}
```

### Expression Statements
```oxide
42;
1 + 2 * 3;
(x + y) / 2;
```

## Architecture

### Lexer (`src/lexer/`)
The lexer tokenizes source code into the following tokens:
- **Literals**: Numbers (`42`), Identifiers (`variable`)
- **Keywords**: `let`
- **Operators**: `=`, `+`, `-`, `*`, `/`
- **Delimiters**: `;`, `(`, `)`, `{`, `}`
- **Special**: `EOF`, `Illegal`

### Parser (`src/parser/`)
The parser uses recursive descent parsing with operator precedence to build an Abstract Syntax Tree (AST):
- **Expressions**: Numbers, identifiers, binary operations, unary operations, grouping
- **Statements**: Let statements, expression statements, block statements
- **Error Recovery**: Synchronization on statement boundaries

## Usage

### As a Library

Add to your `Cargo.toml`:
```toml
[dependencies]
oxide = "0.1.0"
```

Parse source code:
```rust
use oxide::parse_source;

let source = "let x = 1 + 2 * 3;";
match parse_source(source) {
    Ok(program) => {
        println!("Parsed {} statements", program.statements.len());
        for stmt in &program.statements {
            println!("{}", stmt);
        }
    }
    Err(errors) => {
        println!("Parse errors: {}", errors);
    }
}
```

### REPL (Interactive Mode)

Run the interactive REPL:
```bash
cargo run --bin oxide-repl
```

Commands:
- `help` - Show help message
- `quit` - Exit the REPL
- `clear` - Clear screen
- Enter any Oxide code to parse and see the AST

### Examples

Run the demo:
```bash
cargo run --example parser_demo
```

## Project Structure

```
Oxide/
├── src/
│   ├── lib.rs              # Library root
│   ├── lexer/
│   │   ├── mod.rs          # Lexer module
│   │   └── lexer.rs        # Lexer implementation
│   ├── parser/
│   │   ├── mod.rs          # Parser module
│   │   ├── ast.rs          # AST node definitions
│   │   ├── error.rs        # Error types and handling
│   │   └── parse.rs        # Parser implementation
│   └── bin/
│       └── repl.rs         # Interactive REPL
├── examples/
│   └── parser_demo.rs      # Usage examples
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

## Grammar

The language follows this grammar (in EBNF):

```ebnf
program     = statement* ;
statement   = letStmt | blockStmt | exprStmt ;
letStmt     = "let" IDENTIFIER "=" expression ";" ;
blockStmt   = "{" statement* "}" ;
exprStmt    = expression ";" ;

expression  = binary ;
binary      = unary ( ( "+" | "-" | "*" | "/" ) unary )* ;
unary       = ( "-" ) unary | primary ;
primary     = NUMBER | IDENTIFIER | "(" expression ")" ;
```

## Operator Precedence

1. `*`, `/` (highest)
2. `+`, `-` (lowest)

Parentheses can override precedence: `(1 + 2) * 3` vs `1 + 2 * 3`

## Error Handling

The parser provides detailed error messages with position information:
- Unexpected tokens with suggestions
- Missing expressions or semicolons
- Invalid operators
- Synchronization for error recovery

## Testing

Run all tests:
```bash
cargo test
```

Run specific test modules:
```bash
cargo test lexer
cargo test parser
```

## Examples in Action

### Simple Variable Declaration
```oxide
let x = 42;
```
**AST**: `Let { name: "x", value: Number(42) }`

### Complex Expression
```oxide
let result = (1 + 2) * 3 - 4;
```
**AST**: 
```
Let {
  name: "result",
  value: Binary {
    left: Binary {
      left: Grouping(Binary { left: Number(1), op: Add, right: Number(2) }),
      op: Multiply,
      right: Number(3)
    },
    op: Subtract,
    right: Number(4)
  }
}
```

### Block Statement
```oxide
{
    let x = 5;
    let y = x + 10;
    y;
}
```

## Development

Build the project:
```bash
cargo build
```

Run with debug output:
```bash
RUST_LOG=debug cargo run --bin oxide-repl
```

Format code:
```bash
cargo fmt
```

Lint code:
```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Future Enhancements

- [ ] Function declarations and calls
- [ ] Control flow (if/else, loops)
- [ ] More data types (strings, booleans)
- [ ] Variable scoping and environments
- [ ] Type system
- [ ] Code generation/interpretation
- [ ] Standard library
- [ ] Module system