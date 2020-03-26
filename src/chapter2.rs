pub mod parser {
    trait Parser {}
    pub mod infix2suffix;
    mod defparser;          // dead_code
}
pub mod lexer;
pub mod token;
pub mod mtype;
pub mod symbol;