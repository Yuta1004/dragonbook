#[cfg(test)]
mod tests;

use super::super::lexer::Tag;

pub enum Token {
    Num { tag: Tag, num: i32 },
    Word { tag: Tag, lexeme: String }
}

impl Token {
    fn new_num(num: i32) -> Token {
        Token::Num { tag: Tag::Num, num }
    }

    fn new_word(tag: Tag, lexeme: String) -> Token {
        Token::Word { tag, lexeme }
    }
}
