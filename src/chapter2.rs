pub mod parser {
    trait Parser {}
    pub mod infix2suffix;
    pub mod defparser;
}
pub mod lexer;
pub mod token;
pub mod mtype;
pub mod symbol;