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
    Num { tag: Tag, num: i32 },
    Word { tag: Tag, lexeme: String }
}

impl Token {
    pub fn new_num(num: i32) -> Token {
        Token::Num { tag: Tag::Num, num }
    }

    pub fn new_word(tag: Tag, lexeme: String) -> Token {
        Token::Word { tag, lexeme }
    }
}
