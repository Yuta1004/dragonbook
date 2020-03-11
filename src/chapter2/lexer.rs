mod token;

use std::collections::HashMap;

use token::Token;

// scanner
pub struct Lexer {
    line: i32,
    nowon: i32,
    program: String,
    matchTable: HashMap<String, Token>
}

impl Lexer {
    pub fn new(program: String) -> Lexer {
        Lexer { line: 0, nowon: 0, program, matchTable: HashMap::new() }
    }
}
