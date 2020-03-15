#[cfg(test)]
mod tests;

use std::clone::Clone;

#[derive(Clone, PartialEq)]
pub enum Tag {
    Id,
    True,
    False
}

#[derive(Clone)]
pub enum Token {
    NumI32 { num: i32 },
    NumF32 { num: f32 },
    Word { tag: Tag, lexeme: String }
}

impl Token {
    pub fn new_numi32(num: i32) -> Token {
        Token::NumI32 { num }
    }

    pub fn new_numf32(num: f32) -> Token {
        Token::NumF32 { num }
    }

    pub fn new_word(tag: Tag, lexeme: String) -> Token {
        Token::Word { tag, lexeme }
    }
}
