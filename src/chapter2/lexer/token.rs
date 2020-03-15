#[cfg(test)]
mod tests;

use std::clone::Clone;

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Token {
    NumI32 { tag: Tag, num: i32 },
    Word { tag: Tag, lexeme: String }
}

impl Token {
    pub fn new_numi32(num: i32) -> Token {
        Token::NumI32 { tag: Tag::Num, num }
    }

    pub fn new_word(tag: Tag, lexeme: String) -> Token {
        Token::Word { tag, lexeme }
    }
}
