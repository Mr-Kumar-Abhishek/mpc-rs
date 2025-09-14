# Micro Parser Combinators (mpc-rs)

A lightweight and powerful Parser Combinator library for Rust, ported from the C library [mpc](https://github.com/orangeduck/mpc).

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## About

_mpc-rs_ is a parser combinator library that allows you to build powerful parsers using simple, composable building blocks. Parser combinators are functions that take parsers as input and return new parsers as output, enabling you to construct complex parsers from simple ones.

### When to Use mpc-rs

You should consider using _mpc-rs_ when you need to:

* Build a new programming language
* Parse a new data format
* Parse existing programming languages
* Parse existing data formats
* Embed a domain-specific language
* Implement complex text processing

## Features

* **Type-Generic**: Works with any Rust types through `Box<dyn Any>`
* **Predictive, Recursive Descent**: Efficient parsing with backtracking
* **Easy to Integrate**: Single library crate
* **Automatic Error Reporting**: Detailed error messages with position info
* **Memory Safe**: Leverages Rust's ownership system
* **Composable**: Build complex parsers from simple ones

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mpc = { path = "../mpc-rs" }  # Adjust path as needed
```

Or for a published version (when available):

```toml
[dependencies]
mpc = "0.1"
```

## Quick Start

Here's how to create a simple mathematical expression parser:

```rust
use mpc::*;

fn main() {
    // Define basic parsers
    let number = mpc_digits();
    let plus = mpc_char('+');
    let mul = mpc_char('*');

    // Create expression parsers
    let term = mpc_or(vec![number.clone(), mpc_and(vec![mpc_char('('), expression(), mpc_char(')')], mpcf_fst)]);

    let expression = mpc_or(vec![
        mpc_and(vec![term.clone(), plus.clone(), expression()], |_, xs| {
            // Combine: term + expr
            Box::new(format!("(+ {} {})", xs[0].downcast_ref::<String>().unwrap(), xs[2].downcast_ref::<String>().unwrap()))
        }),
        term
    ]);

    // Parse some input
    match mpc_parse("input", "(4 + 2)", &expression) {
        MpcResult::Ok(result) => {
            println!("Parsed: {:?}", result.downcast_ref::<String>());
        }
        MpcResult::Err(err) => {
            err.print();
        }
    }
}
```

## Core Concepts

### Parsers

A parser is a function that takes input and returns either a successfully parsed value or an error. All parsers implement the `MpcParser` struct with different `MpcParserType` variants.

### Combinators

Combinators take parsers and return new parsers:

- **`mpc_and()`**: Sequence multiple parsers
- **`mpc_or()`**: Try alternatives
- **`mpc_many()`**: Zero or more repetitions
- **`mpc_many1()`**: One or more repetitions

### Results

Parsing returns an `MpcResult`:

```rust
pub enum MpcResult {
    Ok(MpcVal),    // Box<dyn Any> containing parsed value
    Err(MpcErr),   // Error with position and expected tokens
}
```

## API Reference

### Basic Parsers

| Function | Description | Example |
|----------|-------------|---------|
| `mpc_any()` | Matches any single character | |
| `mpc_char(c)` | Matches specific character | `mpc_char('a')` |
| `mpc_range(s, e)` | Matches character in range | `mpc_range('0', '9')` |
| `mpc_oneof(s)` | Matches any char in string | `mpc_oneof("abc")` |
| `mpc_noneof(s)` | Matches any char not in string | `mpc_noneof(" \t\n")` |
| `mpc_satisfy(f)` | Matches char satisfying function | `mpc_satisfy(|c| c.is_digit(10))` |
| `mpc_string(s)` | Matches exact string | `mpc_string("hello")` |

### Other Parsers

| Function | Description |
|----------|-------------|
| `mpc_pass()` | Always succeeds, consumes nothing |
| `mpc_fail(msg)` | Always fails with message |
| `mpc_lift(f)` | Consumes nothing, returns function result |
| `mpc_lift_val(val)` | Consumes nothing, returns value |
| `mpc_anchor(f)` | Checks condition without consuming |
| `mpc_state()` | Returns current parser state |

### Combinators

| Function | Description | Example |
|----------|-------------|---------|
| `mpc_and(parsers, fold)` | Sequence parsers | `mpc_and(vec![a, b], fold_fn)` |
| `mpc_or(parsers)` | Alternative parsers | `mpc_or(vec![a, b])` |
| `mpc_many(parser, fold)` | Zero or more | `mpc_many(digit, strfold)` |
| `mpc_many1(parser, fold)` | One or more | `mpc_many1(digit, strfold)` |
| `mpc_count(n, parser, fold)` | Exactly n times | `mpc_count(3, digit, strfold)` |
| `mpc_sepby(parser, sep, fold)` | Separated by separator | `mpc_sepby(item, comma, fold)` |
| `mpc_sepby1(parser, sep, fold)` | One or more separated | `mpc_sepby1(item, comma, fold)` |

### Utility Parsers

| Function | Description |
|----------|-------------|
| `mpc_digit()` | Single digit (0-9) |
| `mpc_digits()` | One or more digits |
| `mpc_alpha()` | Alphabetic character |
| `mpc_alphanum()` | Alphanumeric character |
| `mpc_whitespace()` | Single whitespace char |
| `mpc_whitespaces()` | Zero or more whitespace |
| `mpc_lower()` | Lowercase letter |
| `mpc_upper()` | Uppercase letter |
| `mpc_eoi()` | End of input |
| `mpc_soi()` | Start of input |

### AST Building

| Function | Description | Example |
|----------|-------------|---------|
| `mpca_tag(parser, tag)` | Tag parser result | `mpca_tag(number, "number")` |
| `mpca_root(parser)` | Mark as AST root | `mpca_root(expression)` |

### Fold Functions

| Function | Description |
|----------|-------------|
| `mpcf_strfold` | Concatenate strings |
| `mpcf_fst` | Return first result |
| `mpcf_null` | Return unit |

### Parsing

| Function | Description | Example |
|----------|-------------|---------|
| `mpc_parse(filename, input, parser)` | Parse string input | `mpc_parse("file", "input", &parser)` |

## Examples

### Simple Calculator

```rust
use mpc::*;

fn main() {
    let number = mpc_digits();
    let plus = mpc_char('+');
    let mul = mpc_char('*');

    let expr = mpc_or(vec![
        mpc_and(vec![number.clone(), plus.clone(), expr], |_, xs| {
            let a: i32 = xs[0].downcast_ref::<String>().unwrap().parse().unwrap();
            let b: i32 = xs[2].downcast_ref::<String>().unwrap().parse().unwrap();
            Box::new(a + b)
        }),
        number
    ]);

    match mpc_parse("calc", "1+2", &expr) {
        MpcResult::Ok(val) => println!("Result: {}", val.downcast_ref::<i32>().unwrap()),
        MpcResult::Err(e) => e.print(),
    }
}
```

### JSON-like Parser

```rust
use mpc::*;

// Simplified JSON parser
fn json_value() -> MpcParser {
    mpc_or(vec![
        json_string(),
        json_number(),
        json_bool(),
        mpc_and(vec![mpc_char('{'), mpc_pass()], |_| Box::new("object")),
    ])
}

fn json_string() -> MpcParser {
    mpc_and(vec![
        mpc_char('"'),
        mpc_many(mpc_noneof("\""), mpcf_strfold),
        mpc_char('"')
    ], |_, xs| xs[1].clone())
}

fn json_number() -> MpcParser {
    mpc_digits()
}

fn json_bool() -> MpcParser {
    mpc_or(vec![
        mpc_string("true"),
        mpc_string("false")
    ])
}
```

## Error Handling

_mpc-rs_ provides detailed error information:

```rust
match mpc_parse("input", "invalid", &parser) {
    MpcResult::Ok(result) => {
        // Process result
    }
    MpcResult::Err(err) => {
        println!("Parse error at line {}, column {}: {}",
                err.state.row + 1,
                err.state.col + 1,
                err.failure);
        println!("Expected: {}", err.expected.join(", "));
    }
}
```

## Building and Testing

```bash
# Build the library
cargo build

# Run tests
cargo test

# Run examples
cargo run --bin simple_test
```

## Performance

_mpc-rs_ uses recursive descent with backtracking, making it suitable for most parsing tasks. For maximum performance on LL(1) grammars, consider using predictive parsing techniques.

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure code compiles and tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This library is a port of the excellent [mpc](https://github.com/orangeduck/mpc) C library by Daniel Holden. The original C implementation provided the foundation and inspiration for this Rust version.