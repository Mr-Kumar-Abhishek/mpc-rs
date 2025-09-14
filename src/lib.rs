//! Micro Parser Combinators
//!
//! A lightweight and powerful Parser Combinator library for Rust.
//!
//! This is a port of the C library mpc (https://github.com/orangeduck/mpc)

pub type MpcVal = Box<dyn std::any::Any>;

/// State Type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MpcState {
    pub pos: i64,
    pub row: i64,
    pub col: i64,
    pub term: i32,
}

impl Default for MpcState {
    fn default() -> Self {
        MpcState {
            pos: 0,
            row: 0,
            col: 0,
            term: 0,
        }
    }
}

/// Error Type
#[derive(Debug, Clone)]
pub struct MpcErr {
    pub state: MpcState,
    pub expected_num: i32,
    pub filename: String,
    pub failure: String,
    pub expected: Vec<String>,
    pub received: char,
}

impl MpcErr {
    pub fn new(state: MpcState, expected: Vec<String>, failure: String, received: char) -> Self {
        MpcErr {
            state,
            expected_num: expected.len() as i32,
            filename: String::new(),
            failure,
            expected,
            received,
        }
    }

    pub fn print(&self) {
        println!("Error: {}", self.failure);
        // TODO: Implement full printing
    }
}

/// Result Type
#[derive(Debug)]
pub enum MpcResult {
    Ok(MpcVal),
    Err(MpcErr),
}

/// Parser Type
pub enum MpcParserType {
    Any,  // Matches any character
    Char(char),  // Matches specific character
    Range(char, char),  // Matches character in range
    OneOf(String),  // Matches any char in string
    NoneOf(String),  // Matches any char not in string
    Satisfy(fn(char) -> bool),  // Matches char satisfying function
    String(String),  // Matches exact string
    Pass,  // Always succeeds, consumes no input
    Fail(String),  // Always fails with message
    Lift(fn() -> MpcVal),  // Consumes no input, returns result of function
    LiftVal(fn() -> MpcVal),  // Consumes no input, returns value
    Anchor(fn(char, char) -> bool),  // Consumes no input, checks condition
    State,  // Consumes no input, returns parser state
    // Combinators
    And(Vec<Box<MpcParser>>, fn(i32, Vec<MpcVal>) -> MpcVal),  // Sequence of parsers
    Or(Vec<Box<MpcParser>>),  // Alternative parsers
    Many(Box<MpcParser>, fn(i32, Vec<MpcVal>) -> MpcVal),  // Zero or more
    Many1(Box<MpcParser>, fn(i32, Vec<MpcVal>) -> MpcVal),  // One or more
}

/// Parser
pub struct MpcParser {
    pub name: String,
    pub parser_type: MpcParserType,
}

impl MpcParser {
    pub fn new(name: &str) -> Self {
        MpcParser {
            name: name.to_string(),
            parser_type: MpcParserType::Any, // placeholder
        }
    }
}

/// AST Type
#[derive(Debug)]
pub struct MpcAst {
    pub tag: String,
    pub contents: String,
    pub state: MpcState,
    pub children_num: i32,
    pub children: Vec<Box<MpcAst>>,
}

impl MpcAst {
    pub fn new(tag: &str, contents: &str) -> Self {
        MpcAst {
            tag: tag.to_string(),
            contents: contents.to_string(),
            state: MpcState::default(),
            children_num: 0,
            children: Vec::new(),
        }
    }

    pub fn print(&self) {
        self.print_recursive(0);
    }

    fn print_recursive(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}{}", indent, self.tag);
        if !self.contents.is_empty() {
            println!("{}  \"{}\"", indent, self.contents);
        }
        for child in &self.children {
            child.print_recursive(depth + 1);
        }
    }
}

// Basic Parsers

pub fn mpc_any() -> MpcParser {
    MpcParser {
        name: "any".to_string(),
        parser_type: MpcParserType::Any,
    }
}

pub fn mpc_char(c: char) -> MpcParser {
    MpcParser {
        name: format!("char:{}", c),
        parser_type: MpcParserType::Char(c),
    }
}

pub fn mpc_range(s: char, e: char) -> MpcParser {
    MpcParser {
        name: format!("range:{}-{}", s, e),
        parser_type: MpcParserType::Range(s, e),
    }
}

pub fn mpc_oneof(s: &str) -> MpcParser {
    MpcParser {
        name: format!("oneof:{}", s),
        parser_type: MpcParserType::OneOf(s.to_string()),
    }
}

pub fn mpc_noneof(s: &str) -> MpcParser {
    MpcParser {
        name: format!("noneof:{}", s),
        parser_type: MpcParserType::NoneOf(s.to_string()),
    }
}

pub fn mpc_satisfy(f: fn(char) -> bool) -> MpcParser {
    MpcParser {
        name: "satisfy".to_string(),
        parser_type: MpcParserType::Satisfy(f),
    }
}

pub fn mpc_string(s: &str) -> MpcParser {
    MpcParser {
        name: format!("string:{}", s),
        parser_type: MpcParserType::String(s.to_string()),
    }
}

// Other Parsers

pub fn mpc_pass() -> MpcParser {
    MpcParser {
        name: "pass".to_string(),
        parser_type: MpcParserType::Pass,
    }
}

pub fn mpc_fail(m: &str) -> MpcParser {
    MpcParser {
        name: format!("fail:{}", m),
        parser_type: MpcParserType::Fail(m.to_string()),
    }
}

pub fn mpc_lift(f: fn() -> MpcVal) -> MpcParser {
    MpcParser {
        name: "lift".to_string(),
        parser_type: MpcParserType::Lift(f),
    }
}

pub fn mpc_lift_val(f: fn() -> MpcVal) -> MpcParser {
    MpcParser {
        name: "lift_val".to_string(),
        parser_type: MpcParserType::LiftVal(f),
    }
}

pub fn mpc_anchor(f: fn(char, char) -> bool) -> MpcParser {
    MpcParser {
        name: "anchor".to_string(),
        parser_type: MpcParserType::Anchor(f),
    }
}

pub fn mpc_state() -> MpcParser {
    MpcParser {
        name: "state".to_string(),
        parser_type: MpcParserType::State,
    }
}

// Input Stream
pub struct MpcInput<'a> {
    pub filename: String,
    pub state: MpcState,
    pub string: &'a str,
    pub pos: usize,
}

impl<'a> MpcInput<'a> {
    pub fn new(filename: &str, string: &'a str) -> Self {
        MpcInput {
            filename: filename.to_string(),
            state: MpcState::default(),
            string,
            pos: 0,
        }
    }

    pub fn remaining(&self) -> &str {
        &self.string[self.pos..]
    }

    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
            self.state.pos += 1;
            if c == '\n' {
                self.state.col = 0;
                self.state.row += 1;
            } else {
                self.state.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }
}

impl MpcParser {
    pub fn parse<'a>(&self, input: &mut MpcInput<'a>) -> MpcResult {
        match &self.parser_type {
            MpcParserType::Any => {
                if let Some(c) = input.advance() {
                    MpcResult::Ok(Box::new(c.to_string()))
                } else {
                    MpcResult::Err(MpcErr::new(input.state, vec!["any".to_string()], "unexpected end of input".to_string(), '\0'))
                }
            }
            MpcParserType::Char(expected) => {
                if let Some(c) = input.peek() {
                    if c == *expected {
                        input.advance();
                        MpcResult::Ok(Box::new(c.to_string()))
                    } else {
                        MpcResult::Err(MpcErr::new(input.state, vec![expected.to_string()], format!("expected '{}'", expected), c))
                    }
                } else {
                    MpcResult::Err(MpcErr::new(input.state, vec![expected.to_string()], format!("expected '{}'", expected), '\0'))
                }
            }
            MpcParserType::Range(start, end) => {
                if let Some(c) = input.peek() {
                    if c >= *start && c <= *end {
                        input.advance();
                        MpcResult::Ok(Box::new(c.to_string()))
                    } else {
                        MpcResult::Err(MpcErr::new(input.state, vec![format!("{}-{}", start, end)], format!("expected char in range {}-{}", start, end), c))
                    }
                } else {
                    MpcResult::Err(MpcErr::new(input.state, vec![format!("{}-{}", start, end)], format!("expected char in range {}-{}", start, end), '\0'))
                }
            }
            MpcParserType::OneOf(chars) => {
                if let Some(c) = input.peek() {
                    if chars.contains(c) {
                        input.advance();
                        MpcResult::Ok(Box::new(c.to_string()))
                    } else {
                        MpcResult::Err(MpcErr::new(input.state, vec![chars.clone()], format!("expected one of '{}'", chars), c))
                    }
                } else {
                    MpcResult::Err(MpcErr::new(input.state, vec![chars.clone()], format!("expected one of '{}'", chars), '\0'))
                }
            }
            MpcParserType::NoneOf(chars) => {
                if let Some(c) = input.peek() {
                    if !chars.contains(c) {
                        input.advance();
                        MpcResult::Ok(Box::new(c.to_string()))
                    } else {
                        MpcResult::Err(MpcErr::new(input.state, vec![format!("not {}", chars)], format!("unexpected one of '{}'", chars), c))
                    }
                } else {
                    MpcResult::Ok(Box::new("".to_string())) // EOF is fine
                }
            }
            MpcParserType::Satisfy(f) => {
                if let Some(c) = input.peek() {
                    if f(c) {
                        input.advance();
                        MpcResult::Ok(Box::new(c.to_string()))
                    } else {
                        input.advance();
                        MpcResult::Err(MpcErr::new(input.state, vec!["satisfy".to_string()], "char does not satisfy condition".to_string(), c))
                    }
                } else {
                    MpcResult::Err(MpcErr::new(input.state, vec!["satisfy".to_string()], "end of input".to_string(), '\0'))
                }
            }
            MpcParserType::String(s) => {
                for expected in s.chars() {
                    if let Some(c) = input.peek() {
                        if c == expected {
                            input.advance();
                        } else {
                            return MpcResult::Err(MpcErr::new(input.state, vec![s.clone()], format!("expected '{}'", s), c));
                        }
                    } else {
                        return MpcResult::Err(MpcErr::new(input.state, vec![s.clone()], format!("expected '{}'", s), '\0'));
                    }
                }
                MpcResult::Ok(Box::new(s.clone()))
            }
            MpcParserType::Pass => {
                MpcResult::Ok(Box::new(()))
            }
            MpcParserType::Fail(msg) => {
                MpcResult::Err(MpcErr::new(input.state, vec![], msg.clone(), '\0'))
            }
            MpcParserType::Lift(f) => {
                MpcResult::Ok(f())
            }
            MpcParserType::LiftVal(f) => {
                MpcResult::Ok(f())
            }
            MpcParserType::Anchor(_f) => {
                // TODO: implement anchor
                MpcResult::Ok(Box::new(()))
            }
            MpcParserType::State => {
                MpcResult::Ok(Box::new(input.state))
            }
            MpcParserType::And(ref parsers, fold) => {
                let mut results = Vec::new();
                for parser in parsers {
                    match parser.parse(input) {
                        MpcResult::Ok(val) => results.push(val),
                        MpcResult::Err(e) => return MpcResult::Err(e),
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
            MpcParserType::Or(ref parsers) => {
                for parser in parsers {
                    match parser.parse(input) {
                        MpcResult::Ok(val) => return MpcResult::Ok(val),
                        MpcResult::Err(_) => continue,
                    }
                }
                MpcResult::Err(MpcErr::new(input.state, vec!["or".to_string()], "no alternatives matched".to_string(), '\0'))
            }
            MpcParserType::Many(ref parser, fold) => {
                let mut results = Vec::new();
                loop {
                    match parser.parse(input) {
                        MpcResult::Ok(val) => results.push(val),
                        MpcResult::Err(_) => break,
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
            MpcParserType::Many1(ref parser, fold) => {
                let mut results = Vec::new();
                let first = match parser.parse(input) {
                    MpcResult::Ok(val) => val,
                    MpcResult::Err(e) => return MpcResult::Err(e),
                };
                results.push(first);
                loop {
                    match parser.parse(input) {
                        MpcResult::Ok(val) => results.push(val),
                        MpcResult::Err(_) => break,
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
        }
    }
}

// Main parsing function
pub fn mpc_parse(filename: &str, string: &str, parser: &MpcParser) -> MpcResult {
    let mut input = MpcInput::new(filename, string);
    parser.parse(&mut input)
}

// Combinator Parsers

pub fn mpc_and(parsers: Vec<MpcParser>, fold: fn(i32, Vec<MpcVal>) -> MpcVal) -> MpcParser {
    MpcParser {
        name: "and".to_string(),
        parser_type: MpcParserType::And(parsers.into_iter().map(Box::new).collect(), fold),
    }
}

pub fn mpc_or(parsers: Vec<MpcParser>) -> MpcParser {
    MpcParser {
        name: "or".to_string(),
        parser_type: MpcParserType::Or(parsers.into_iter().map(Box::new).collect()),
    }
}

pub fn mpc_many(parser: MpcParser, fold: fn(i32, Vec<MpcVal>) -> MpcVal) -> MpcParser {
    MpcParser {
        name: "many".to_string(),
        parser_type: MpcParserType::Many(Box::new(parser), fold),
    }
}

pub fn mpc_many1(parser: MpcParser, fold: fn(i32, Vec<MpcVal>) -> MpcVal) -> MpcParser {
    MpcParser {
        name: "many1".to_string(),
        parser_type: MpcParserType::Many1(Box::new(parser), fold),
    }
}
