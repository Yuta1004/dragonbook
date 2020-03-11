mod token;

// Tag
pub enum Tag {
    Plus,
    Sub,
    Mul,
    Div,
    Num,
    Id,
    True,
    False
}

// scanner
pub struct Lexer {
    line: i32,
    nowon: i32,
    program: String
}

impl Lexer {
    pub fn new(program: String) -> Lexer {
        Lexer { line: 0, nowon: 0, program }
    }
}
