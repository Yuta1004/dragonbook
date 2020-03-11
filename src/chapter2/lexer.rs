mod token;

use std::collections::HashMap;

use token::Token;

// scanner
pub struct Lexer<'a> {
    line: i32,
    nowon: i32,
    program: String,
    matchTable: HashMap<String, &'a Token>
}

impl<'a> Lexer<'a> {
    pub fn new(program: String) -> Lexer<'a> {
        Lexer { line: 0, nowon: 0, program, matchTable: HashMap::new() }
    }

    fn reserve(&mut self, tag: String, token: &'a Token) {
        self.matchTable.insert(tag, token);
    }
}
