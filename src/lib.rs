//! Micro Parser Combinators
//!
//! A lightweight and powerful Parser Combinator library for Rust.
//!
//! This is a port of the C library mpc (<https://github.com/orangeduck/mpc>)

pub type MpcVal = Box<dyn std::any::Any>;

/// State Type
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MpcState {
    pub pos: i64,
    pub row: i64,
    pub col: i64,
    pub term: i32,
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
#[derive(Clone)]
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
    Count(i32, Box<MpcParser>, fn(i32, Vec<MpcVal>) -> MpcVal),  // Exactly n times
    SepBy(Box<MpcParser>, Box<MpcParser>, fn(i32, Vec<MpcVal>) -> MpcVal),  // Separated by
    SepBy1(Box<MpcParser>, Box<MpcParser>, fn(i32, Vec<MpcVal>) -> MpcVal),  // One or more separated by
    // AST Building
    Tag(Box<MpcParser>, String),  // Add tag to result
    Root(Box<MpcParser>),  // Make root of AST
}

/// Parser
#[derive(Clone)]
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
#[derive(Debug, Clone)]
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
                    MpcResult::Err(MpcErr::new(input.state, vec![format!("not {}", chars)], "unexpected end of input".to_string(), '\0'))
                }
            }
            MpcParserType::Satisfy(f) => {
                if let Some(c) = input.peek() {
                    if f(c) {
                        input.advance();
                        MpcResult::Ok(Box::new(c.to_string()))
                    } else {
                        MpcResult::Err(MpcErr::new(input.state, vec!["satisfy".to_string()], "char does not satisfy condition".to_string(), c))
                    }
                } else {
                    MpcResult::Err(MpcErr::new(input.state, vec!["satisfy".to_string()], "end of input".to_string(), '\0'))
                }
            }
            MpcParserType::String(s) => {
                let saved_pos = input.pos;
                let saved_state = input.state;
                for expected in s.chars() {
                    if let Some(c) = input.peek() {
                        if c == expected {
                            input.advance();
                        } else {
                            input.pos = saved_pos;
                            input.state = saved_state;
                            return MpcResult::Err(MpcErr::new(saved_state, vec![s.clone()], format!("expected '{}'", s), c));
                        }
                    } else {
                        input.pos = saved_pos;
                        input.state = saved_state;
                        return MpcResult::Err(MpcErr::new(saved_state, vec![s.clone()], format!("expected '{}'", s), '\0'));
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
            MpcParserType::Anchor(f) => {
                let prev = if input.pos == 0 {
                    '\0'
                } else {
                    input.string[..input.pos].chars().next_back().unwrap_or('\0')
                };
                let next = input.peek().unwrap_or('\0');
                if f(prev, next) {
                    MpcResult::Ok(Box::new(()))
                } else {
                    MpcResult::Err(MpcErr::new(
                        input.state,
                        vec![self.name.clone()],
                        format!("expected {}", self.name),
                        next,
                    ))
                }
            }
            MpcParserType::State => {
                MpcResult::Ok(Box::new(input.state))
            }
            MpcParserType::And(ref parsers, fold) => {
                let saved_pos = input.pos;
                let saved_state = input.state;
                let mut results = Vec::new();
                for parser in parsers {
                    match parser.parse(input) {
                        MpcResult::Ok(val) => results.push(val),
                        MpcResult::Err(e) => {
                            input.pos = saved_pos;
                            input.state = saved_state;
                            return MpcResult::Err(e);
                        }
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
            MpcParserType::Tag(ref parser, ref tag) => {
                match parser.parse(input) {
                    MpcResult::Ok(val) => {
                        // Create AST node with tag
                        let ast = MpcAst::new(tag, &format!("{:?}", val));
                        MpcResult::Ok(Box::new(ast))
                    }
                    MpcResult::Err(e) => MpcResult::Err(e),
                }
            }
            MpcParserType::Root(ref parser) => {
                match parser.parse(input) {
                    MpcResult::Ok(val) => {
                        // Make it root - for now, just return as is since AST handling needs refactoring
                        MpcResult::Ok(val)
                    }
                    MpcResult::Err(e) => MpcResult::Err(e),
                }
            }
            MpcParserType::Or(ref parsers) => {
                let mut last_err = None;
                for parser in parsers {
                    let saved_pos = input.pos;
                    let saved_state = input.state;
                    match parser.parse(input) {
                        MpcResult::Ok(val) => return MpcResult::Ok(val),
                        MpcResult::Err(e) => {
                            input.pos = saved_pos;
                            input.state = saved_state;
                            last_err = Some(e);
                        }
                    }
                }
                MpcResult::Err(last_err.unwrap_or_else(|| MpcErr::new(input.state, vec!["or".to_string()], "no alternatives matched".to_string(), '\0')))
            }
            MpcParserType::Many(ref parser, fold) => {
                let mut results = Vec::new();
                loop {
                    let before = input.pos;
                    match parser.parse(input) {
                        MpcResult::Ok(val) => {
                            results.push(val);
                            if input.pos == before {
                                break;
                            }
                        }
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
                    let before = input.pos;
                    match parser.parse(input) {
                        MpcResult::Ok(val) => {
                            results.push(val);
                            if input.pos == before {
                                break;
                            }
                        }
                        MpcResult::Err(_) => break,
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
            MpcParserType::Count(n, ref parser, fold) => {
                let saved_pos = input.pos;
                let saved_state = input.state;
                let mut results = Vec::new();
                for _ in 0..*n {
                    match parser.parse(input) {
                        MpcResult::Ok(val) => results.push(val),
                        MpcResult::Err(e) => {
                            input.pos = saved_pos;
                            input.state = saved_state;
                            return MpcResult::Err(e);
                        }
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
            MpcParserType::SepBy(ref parser, ref sep, fold) => {
                let mut results = Vec::new();
                // Optional first parser
                if let MpcResult::Ok(val) = parser.parse(input) {
                    results.push(val);
                    loop {
                        let saved_pos = input.pos;
                        let saved_state = input.state;
                        match sep.parse(input) {
                            MpcResult::Ok(_) => {
                                match parser.parse(input) {
                                    MpcResult::Ok(val) => results.push(val),
                                    MpcResult::Err(_) => {
                                        input.pos = saved_pos;
                                        input.state = saved_state;
                                        break;
                                    }
                                }
                            }
                            MpcResult::Err(_) => break,
                        }
                    }
                }
                let folded = fold(results.len() as i32, results);
                MpcResult::Ok(folded)
            }
            MpcParserType::SepBy1(ref parser, ref sep, fold) => {
                let mut results = Vec::new();
                // First parser required
                let first = match parser.parse(input) {
                    MpcResult::Ok(val) => val,
                    MpcResult::Err(e) => return MpcResult::Err(e),
                };
                results.push(first);
                loop {
                    let saved_pos = input.pos;
                    let saved_state = input.state;
                    match sep.parse(input) {
                        MpcResult::Ok(_) => {
                            match parser.parse(input) {
                                MpcResult::Ok(val) => results.push(val),
                                MpcResult::Err(_) => {
                                    input.pos = saved_pos;
                                    input.state = saved_state;
                                    break;
                                }
                            }
                        }
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

pub fn mpc_maybe(parser: MpcParser) -> MpcParser {
    MpcParser {
        name: "maybe".to_string(),
        parser_type: MpcParserType::Or(vec![Box::new(parser), Box::new(mpc_pass())]),
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

pub fn mpc_count(n: i32, parser: MpcParser, fold: fn(i32, Vec<MpcVal>) -> MpcVal) -> MpcParser {
    MpcParser {
        name: format!("count:{}", n),
        parser_type: MpcParserType::Count(n, Box::new(parser), fold),
    }
}

pub fn mpc_sepby(parser: MpcParser, sep: MpcParser, fold: fn(i32, Vec<MpcVal>) -> MpcVal) -> MpcParser {
    MpcParser {
        name: "sepby".to_string(),
        parser_type: MpcParserType::SepBy(Box::new(parser), Box::new(sep), fold),
    }
}

pub fn mpc_sepby1(parser: MpcParser, sep: MpcParser, fold: fn(i32, Vec<MpcVal>) -> MpcVal) -> MpcParser {
    MpcParser {
        name: "sepby1".to_string(),
        parser_type: MpcParserType::SepBy1(Box::new(parser), Box::new(sep), fold),
    }
}

// Common Fold Functions

pub fn mpcf_strfold(_n: i32, xs: Vec<MpcVal>) -> MpcVal {
    let mut result = String::new();
    for x in xs {
        if let Ok(s) = x.downcast::<String>() {
            result.push_str(&s);
        }
    }
    Box::new(result)
}

pub fn mpcf_fst(_n: i32, xs: Vec<MpcVal>) -> MpcVal {
    if xs.is_empty() {
        Box::new(())
    } else {
        xs.into_iter().next().unwrap()
    }
}

pub fn mpcf_null(_n: i32, _xs: Vec<MpcVal>) -> MpcVal {
    Box::new(())
}

// Utility Parsers

pub fn mpc_eoi() -> MpcParser {
    MpcParser {
        name: "eoi".to_string(),
        parser_type: MpcParserType::Anchor(|_prev, next| next == '\0'),
    }
}

pub fn mpc_soi() -> MpcParser {
    MpcParser {
        name: "soi".to_string(),
        parser_type: MpcParserType::Anchor(|prev, _next| prev == '\0'),
    }
}

pub fn mpc_boundary() -> MpcParser {
    mpc_boundary_newline()
}

pub fn mpc_boundary_newline() -> MpcParser {
    MpcParser {
        name: "boundary_newline".to_string(),
        parser_type: MpcParserType::Anchor(|prev, next| {
            (prev == '\0' || (!prev.is_alphanumeric() && prev != '_')) &&
            (next == '\0' || (!next.is_alphanumeric() && next != '_'))
        }),
    }
}

pub fn mpc_whitespace() -> MpcParser {
    mpc_oneof(" \t\n\r")
}

pub fn mpc_whitespaces() -> MpcParser {
    mpc_many(mpc_whitespace(), mpcf_strfold)
}

pub fn mpc_blank() -> MpcParser {
    mpc_oneof(" \t")
}

pub fn mpc_newline() -> MpcParser {
    mpc_char('\n')
}

pub fn mpc_tab() -> MpcParser {
    mpc_char('\t')
}

pub fn mpc_escape() -> MpcParser {
    mpc_char('\\')
}

pub fn mpc_digit() -> MpcParser {
    mpc_range('0', '9')
}

pub fn mpc_hexdigit() -> MpcParser {
    mpc_or(vec![mpc_range('0', '9'), mpc_range('a', 'f'), mpc_range('A', 'F')])
}

pub fn mpc_octdigit() -> MpcParser {
    mpc_range('0', '7')
}

pub fn mpc_digits() -> MpcParser {
    mpc_many1(mpc_digit(), mpcf_strfold)
}

pub fn mpc_hexdigits() -> MpcParser {
    mpc_many1(mpc_hexdigit(), mpcf_strfold)
}

pub fn mpc_octdigits() -> MpcParser {
    mpc_many1(mpc_octdigit(), mpcf_strfold)
}

pub fn mpc_lower() -> MpcParser {
    mpc_range('a', 'z')
}

pub fn mpc_upper() -> MpcParser {
    mpc_range('A', 'Z')
}

pub fn mpc_alpha() -> MpcParser {
    mpc_or(vec![mpc_lower(), mpc_upper()])
}

pub fn mpc_underscore() -> MpcParser {
    mpc_char('_')
}

pub fn mpc_alphanum() -> MpcParser {
    mpc_or(vec![mpc_alpha(), mpc_digit()])
}

pub fn mpc_int() -> MpcParser {
    mpc_and(vec![mpc_maybe(mpc_oneof("+-")), mpc_digits()], mpcf_strfold)
}

pub fn mpc_hex() -> MpcParser {
    mpc_and(
        vec![mpc_or(vec![mpc_string("0x"), mpc_string("0X")]), mpc_hexdigits()],
        mpcf_strfold,
    )
}

pub fn mpc_oct() -> MpcParser {
    mpc_and(
        vec![
            mpc_or(vec![mpc_string("0o"), mpc_string("0O"), mpc_string("0")]),
            mpc_octdigits(),
        ],
        mpcf_strfold,
    )
}

pub fn mpc_number() -> MpcParser {
    mpc_or(vec![mpc_hex(), mpc_oct(), mpc_int()])
}

pub fn mpc_real() -> MpcParser {
    let exp = mpc_and(
        vec![mpc_oneof("eE"), mpc_maybe(mpc_oneof("+-")), mpc_digits()],
        mpcf_strfold,
    );
    let frac_with_opt_exp = mpc_and(
        vec![mpc_char('.'), mpc_digits(), mpc_maybe(exp.clone())],
        mpcf_strfold,
    );
    mpc_and(
        vec![
            mpc_maybe(mpc_oneof("+-")),
            mpc_digits(),
            mpc_or(vec![frac_with_opt_exp, exp]),
        ],
        mpcf_strfold,
    )
}

pub fn mpc_float() -> MpcParser {
    mpc_and(vec![mpc_real(), mpc_maybe(mpc_oneof("fF"))], mpcf_strfold)
}

pub fn mpc_escape_seq() -> MpcParser {
    mpc_and(vec![mpc_escape(), mpc_any()], mpcf_strfold)
}

fn mpc_delimited_lit(delim: char, disallowed: &str) -> MpcParser {
    mpc_and(
        vec![
            mpc_char(delim),
            mpc_many(mpc_or(vec![mpc_escape_seq(), mpc_noneof(disallowed)]), mpcf_strfold),
            mpc_char(delim),
        ],
        mpcf_strfold,
    )
}

pub fn mpc_char_lit() -> MpcParser {
    mpc_and(
        vec![
            mpc_char('\''),
            mpc_or(vec![mpc_escape_seq(), mpc_noneof("'\\")]),
            mpc_char('\''),
        ],
        mpcf_strfold,
    )
}

pub fn mpc_string_lit() -> MpcParser {
    mpc_delimited_lit('"', "\"\\")
}

pub fn mpc_regex_lit() -> MpcParser {
    mpc_delimited_lit('/', "/\\")
}

pub fn mpc_ident() -> MpcParser {
    mpc_and(
        vec![
            mpc_or(vec![mpc_alpha(), mpc_underscore()]),
            mpc_many(mpc_or(vec![mpc_alphanum(), mpc_underscore()]), mpcf_strfold),
        ],
        mpcf_strfold,
    )
}

pub fn mpca_tag(parser: MpcParser, tag: &str) -> MpcParser {
    MpcParser {
        name: format!("tag:{}", tag),
        parser_type: MpcParserType::Tag(Box::new(parser), tag.to_string()),
    }
}

pub fn mpca_root(parser: MpcParser) -> MpcParser {
    MpcParser {
        name: "root".to_string(),
        parser_type: MpcParserType::Root(Box::new(parser)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lift_answer() -> MpcVal {
        Box::new(42_i32)
    }

    #[test]
    fn basic_char_any_range_oneof_primitives() {
        let mut input = MpcInput::new("test", "a5x");
        assert_eq!(
            mpc_any()
                .parse(&mut input)
                .expect_ok_str(),
            Some("a")
        );
        assert_eq!(
            mpc_range('0', '9')
                .parse(&mut input)
                .expect_ok_str(),
            Some("5")
        );
        assert_eq!(
            mpc_oneof("xyz")
                .parse(&mut input)
                .expect_ok_str(),
            Some("x")
        );
        assert!(matches!(mpc_any().parse(&mut input), MpcResult::Err(_)));

        assert_eq!(
            mpc_parse("test", "z", &mpc_char('z')).expect_ok_str(),
            Some("z")
        );
        assert!(matches!(mpc_parse("test", "a", &mpc_char('z')), MpcResult::Err(_)));
    }

    #[test]
    fn noneof_matches_char_and_fails_at_end_of_input() {
        let parser = mpc_noneof("abc");
        assert_eq!(mpc_parse("test", "z", &parser).expect_ok_str(), Some("z"));
        assert!(matches!(mpc_parse("test", "a", &parser), MpcResult::Err(_)));
        assert!(matches!(mpc_parse("test", "", &parser), MpcResult::Err(_)));
    }

    #[test]
    fn satisfy_failure_does_not_advance_input() {
        let parser = mpc_satisfy(|c| c.is_ascii_digit());
        let mut input = MpcInput::new("test", "abc");

        let res = parser.parse(&mut input);
        assert!(matches!(res, MpcResult::Err(_)));
        assert_eq!(input.pos, 0);
        assert_eq!(input.state.col, 0);
        assert_eq!(mpc_parse("test", "7", &parser).expect_ok_str(), Some("7"));
    }

    #[test]
    fn string_partial_match_backtracks() {
        let parser = mpc_string("hello");
        let mut input = MpcInput::new("test", "hel");

        let res = parser.parse(&mut input);
        assert!(matches!(res, MpcResult::Err(_)));
        assert_eq!(input.pos, 0);
        assert_eq!(input.state.pos, 0);
        assert_eq!(input.remaining(), "hel");

        assert_eq!(
            mpc_parse("test", "hello!", &parser).expect_ok_str(),
            Some("hello")
        );
    }

    #[test]
    fn control_parsers_pass_fail_lift_and_state() {
        assert!(matches!(mpc_parse("test", "abc", &mpc_pass()), MpcResult::Ok(_)));
        assert!(matches!(mpc_parse("test", "abc", &mpc_fail("boom")), MpcResult::Err(_)));

        match mpc_parse("test", "", &mpc_lift(lift_answer)) {
            MpcResult::Ok(v) => assert_eq!(v.downcast_ref::<i32>(), Some(&42)),
            MpcResult::Err(e) => panic!("unexpected err: {}", e.failure),
        }

        match mpc_parse("test", "", &mpc_lift_val(lift_answer)) {
            MpcResult::Ok(v) => assert_eq!(v.downcast_ref::<i32>(), Some(&42)),
            MpcResult::Err(e) => panic!("unexpected err: {}", e.failure),
        }

        let mut input = MpcInput::new("test", "ab");
        let _ = mpc_char('a').parse(&mut input);
        match mpc_state().parse(&mut input) {
            MpcResult::Ok(v) => {
                let st = v.downcast_ref::<MpcState>().copied();
                assert_eq!(st.map(|s| (s.pos, s.col)), Some((1, 1)));
            }
            MpcResult::Err(e) => panic!("unexpected err: {}", e.failure),
        }
    }

    #[test]
    fn or_backtracks_after_partial_branch_failure() {
        let parser = mpc_or(vec![mpc_string("abc"), mpc_string("abd")]);
        assert_eq!(mpc_parse("test", "abd", &parser).expect_ok_str(), Some("abd"));
    }

    #[test]
    fn many_and_many1_handle_repetitions_and_zero_width_guard() {
        let zero_width = mpc_many(mpc_pass(), mpcf_strfold);
        assert_eq!(mpc_parse("test", "abc", &zero_width).expect_ok_str(), Some(""));

        let zero_width_1 = mpc_many1(mpc_pass(), mpcf_strfold);
        assert_eq!(mpc_parse("test", "abc", &zero_width_1).expect_ok_str(), Some(""));

        let letters = mpc_many1(mpc_alpha(), mpcf_strfold);
        assert_eq!(mpc_parse("test", "hi1", &letters).expect_ok_str(), Some("hi"));
        assert!(matches!(mpc_parse("test", "1", &letters), MpcResult::Err(_)));
    }

    #[test]
    fn sepby_and_sepby1_backtrack_trailing_separator() {
        let parser = mpc_sepby(mpc_digits(), mpc_char(','), mpcf_strfold);
        let mut input = MpcInput::new("test", "12,34,");
        assert_eq!(parser.parse(&mut input).expect_ok_str(), Some("1234"));
        assert_eq!(input.remaining(), ",");

        assert_eq!(mpc_parse("test", "", &parser).expect_ok_str(), Some(""));

        let parser1 = mpc_sepby1(mpc_digits(), mpc_char(','), mpcf_strfold);
        let mut input1 = MpcInput::new("test", "9,");
        assert_eq!(parser1.parse(&mut input1).expect_ok_str(), Some("9"));
        assert_eq!(input1.remaining(), ",");
        assert!(matches!(mpc_parse("test", "", &parser1), MpcResult::Err(_)));
    }

    #[test]
    fn fold_functions_and_character_utilities() {
        let fst_parser = mpc_and(vec![mpc_char('a'), mpc_char('b')], mpcf_fst);
        assert_eq!(mpc_parse("test", "ab", &fst_parser).expect_ok_str(), Some("a"));

        let null_parser = mpc_and(vec![mpc_char('a'), mpc_char('b')], mpcf_null);
        match mpc_parse("test", "ab", &null_parser) {
            MpcResult::Ok(v) => assert!(v.is::<()>()),
            MpcResult::Err(e) => panic!("unexpected err: {}", e.failure),
        }

        assert_eq!(mpc_parse("test", "019", &mpc_digits()).expect_ok_str(), Some("019"));
        assert_eq!(mpc_parse("test", "1aF9", &mpc_hexdigits()).expect_ok_str(), Some("1aF9"));
        assert_eq!(mpc_parse("test", "0758", &mpc_octdigits()).expect_ok_str(), Some("075"));
        assert_eq!(
            mpc_parse("test", "a1B", &mpc_many1(mpc_alphanum(), mpcf_strfold)).expect_ok_str(),
            Some("a1B")
        );
        assert_eq!(
            mpc_parse("test", " \t\nx", &mpc_whitespaces()).expect_ok_str(),
            Some(" \t\n")
        );
    }

    #[test]
    fn utf8_input_tracks_row_col_and_byte_pos() {
        let parser = mpc_many(mpc_any(), mpcf_strfold);
        let mut input = MpcInput::new("test", "a\né");

        assert_eq!(parser.parse(&mut input).expect_ok_str(), Some("a\né"));
        assert_eq!(input.state.pos, 3);
        assert_eq!(input.state.row, 1);
        assert_eq!(input.state.col, 1);
        assert_eq!(input.pos, "a\né".len());
    }

    // --- Sprint 4 (US-04 & US-05) TDD Specifications ---

    #[test]
    fn soi_anchor_matches_only_at_start_of_input() {
        let mut input = MpcInput::new("test", "ab");
        assert!(matches!(mpc_soi().parse(&mut input), MpcResult::Ok(_)));

        let _ = mpc_char('a').parse(&mut input);
        assert!(matches!(mpc_soi().parse(&mut input), MpcResult::Err(_)));
    }

    #[test]
    fn eoi_anchor_matches_only_at_end_of_input() {
        let parser = mpc_and(vec![mpc_digits(), mpc_eoi()], mpcf_fst);
        assert_eq!(mpc_parse("test", "123", &parser).expect_ok_str(), Some("123"));
        assert!(matches!(mpc_parse("test", "123abc", &parser), MpcResult::Err(_)));
    }

    #[test]
    fn boundary_newline_anchor_checks_non_word_transitions() {
        let mut input_space = MpcInput::new("test", " hello");
        assert!(matches!(mpc_boundary_newline().parse(&mut input_space), MpcResult::Ok(_)));

        let mut input_word = MpcInput::new("test", "hello");
        assert!(matches!(mpc_boundary_newline().parse(&mut input_word), MpcResult::Err(_)));

        let mut input_mid = MpcInput::new("test", "  a");
        let _ = mpc_char(' ').parse(&mut input_mid);
        assert!(matches!(mpc_boundary_newline().parse(&mut input_mid), MpcResult::Ok(_)));
        let _ = mpc_char(' ').parse(&mut input_mid);
        assert!(matches!(mpc_boundary_newline().parse(&mut input_mid), MpcResult::Err(_)));
    }

    #[test]
    fn and_restores_input_state_when_later_parser_fails() {
        let parser = mpc_and(vec![mpc_char('a'), mpc_char('b')], mpcf_strfold);
        let mut input = MpcInput::new("test", "ax");

        let res = parser.parse(&mut input);
        assert!(matches!(res, MpcResult::Err(_)));
        assert_eq!(input.pos, 0);
        assert_eq!(input.state.pos, 0);
        assert_eq!(input.state.col, 0);
        assert_eq!(input.remaining(), "ax");
    }

    #[test]
    fn count_restores_input_state_when_later_iteration_fails() {
        let parser = mpc_count(3, mpc_char('a'), mpcf_strfold);
        let mut input = MpcInput::new("test", "aab");

        let res = parser.parse(&mut input);
        assert!(matches!(res, MpcResult::Err(_)));
        assert_eq!(input.pos, 0);
        assert_eq!(input.state.pos, 0);
        assert_eq!(input.state.col, 0);
        assert_eq!(input.remaining(), "aab");
    }

    // --- Sprint 5 (US-07) Token, Literal & Maybe Combinator Tests ---

    #[test]
    fn int_hex_oct_and_number_parsers() {
        assert_eq!(mpc_parse("test", "42", &mpc_int()).expect_ok_str(), Some("42"));
        assert_eq!(mpc_parse("test", "-42", &mpc_int()).expect_ok_str(), Some("-42"));
        assert_eq!(mpc_parse("test", "+7", &mpc_int()).expect_ok_str(), Some("+7"));

        let mut bad_int = MpcInput::new("test", "-abc");
        assert!(matches!(mpc_int().parse(&mut bad_int), MpcResult::Err(_)));
        assert_eq!(bad_int.pos, 0);

        assert_eq!(mpc_parse("test", "0x1aF", &mpc_hex()).expect_ok_str(), Some("0x1aF"));
        assert_eq!(mpc_parse("test", "0XFF", &mpc_hex()).expect_ok_str(), Some("0XFF"));
        let mut bad_hex = MpcInput::new("test", "0xzz");
        assert!(matches!(mpc_hex().parse(&mut bad_hex), MpcResult::Err(_)));
        assert_eq!(bad_hex.pos, 0);

        assert_eq!(mpc_parse("test", "0o755", &mpc_oct()).expect_ok_str(), Some("0o755"));
        assert_eq!(mpc_parse("test", "0O12", &mpc_oct()).expect_ok_str(), Some("0O12"));
        assert_eq!(mpc_parse("test", "0755", &mpc_oct()).expect_ok_str(), Some("0755"));

        assert_eq!(mpc_parse("test", "0x1aF", &mpc_number()).expect_ok_str(), Some("0x1aF"));
        assert_eq!(mpc_parse("test", "0o755", &mpc_number()).expect_ok_str(), Some("0o755"));
        assert_eq!(mpc_parse("test", "-99", &mpc_number()).expect_ok_str(), Some("-99"));
        assert_eq!(mpc_parse("test", "0", &mpc_number()).expect_ok_str(), Some("0"));
    }

    #[test]
    fn real_and_float_parsers() {
        assert_eq!(mpc_parse("test", "3.14", &mpc_real()).expect_ok_str(), Some("3.14"));
        assert_eq!(mpc_parse("test", "-0.5", &mpc_real()).expect_ok_str(), Some("-0.5"));
        assert_eq!(mpc_parse("test", "2.5e-3", &mpc_real()).expect_ok_str(), Some("2.5e-3"));
        assert_eq!(mpc_parse("test", "1e10", &mpc_real()).expect_ok_str(), Some("1e10"));
        assert!(matches!(mpc_parse("test", "42", &mpc_real()), MpcResult::Err(_)));

        let mut bad_real = MpcInput::new("test", "3.x");
        assert!(matches!(mpc_real().parse(&mut bad_real), MpcResult::Err(_)));
        assert_eq!(bad_real.pos, 0);

        assert_eq!(mpc_parse("test", "3.14f", &mpc_float()).expect_ok_str(), Some("3.14f"));
        assert_eq!(mpc_parse("test", "2.0F", &mpc_float()).expect_ok_str(), Some("2.0F"));
        assert_eq!(mpc_parse("test", "2.0", &mpc_float()).expect_ok_str(), Some("2.0"));
    }

    #[test]
    fn char_string_and_regex_literal_parsers() {
        assert_eq!(mpc_parse("test", "'a'", &mpc_char_lit()).expect_ok_str(), Some("'a'"));
        assert_eq!(mpc_parse("test", "'\\n'", &mpc_char_lit()).expect_ok_str(), Some("'\\n'"));
        assert_eq!(mpc_parse("test", "'\\''", &mpc_char_lit()).expect_ok_str(), Some("'\\''"));

        assert_eq!(mpc_parse("test", "\"hello\"", &mpc_string_lit()).expect_ok_str(), Some("\"hello\""));
        assert_eq!(mpc_parse("test", "\"\"", &mpc_string_lit()).expect_ok_str(), Some("\"\""));
        assert_eq!(
            mpc_parse("test", "\"a\\\"b\\n\"", &mpc_string_lit()).expect_ok_str(),
            Some("\"a\\\"b\\n\"")
        );

        let mut unterminated_str = MpcInput::new("test", "\"hello");
        assert!(matches!(mpc_string_lit().parse(&mut unterminated_str), MpcResult::Err(_)));
        assert_eq!(unterminated_str.pos, 0);

        assert_eq!(mpc_parse("test", "/[a-z]+/", &mpc_regex_lit()).expect_ok_str(), Some("/[a-z]+/"));
        assert_eq!(mpc_parse("test", "/a\\/b/", &mpc_regex_lit()).expect_ok_str(), Some("/a\\/b/"));

        let mut unterminated_re = MpcInput::new("test", "/abc");
        assert!(matches!(mpc_regex_lit().parse(&mut unterminated_re), MpcResult::Err(_)));
        assert_eq!(unterminated_re.pos, 0);
    }

    #[test]
    fn ident_and_maybe_parsers() {
        assert_eq!(mpc_parse("test", "foo_bar123", &mpc_ident()).expect_ok_str(), Some("foo_bar123"));
        assert_eq!(mpc_parse("test", "_temp", &mpc_ident()).expect_ok_str(), Some("_temp"));
        assert!(matches!(mpc_parse("test", "1abc", &mpc_ident()), MpcResult::Err(_)));

        let opt_sign = mpc_maybe(mpc_char('-'));
        let cloned = opt_sign.clone();
        assert_eq!(
            mpc_parse("test", "-5", &mpc_and(vec![opt_sign, mpc_digits()], mpcf_strfold)).expect_ok_str(),
            Some("-5")
        );
        assert_eq!(
            mpc_parse("test", "5", &mpc_and(vec![cloned, mpc_digits()], mpcf_strfold)).expect_ok_str(),
            Some("5")
        );
    }

    trait ExpectOkStr {
        fn expect_ok_str(&self) -> Option<&str>;
    }

    impl ExpectOkStr for MpcResult {
        fn expect_ok_str(&self) -> Option<&str> {
            match self {
                MpcResult::Ok(val) => val.downcast_ref::<String>().map(String::as_str),
                MpcResult::Err(_) => None,
            }
        }
    }
}
