use std::clone::Clone;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub enum Tag {
    UpperThanL,
    UpperThanR,
    UpperEqThanL,
    UpperEqThanR,
    Equal,
    NotEqual,
    Id,
    True,
    False,
    None
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

    pub fn new_word(tag: Tag, lexeme: &str) -> Token {
        Token::Word { tag, lexeme: String::from_str(lexeme).unwrap() }
    }
}

#[cfg(test)]
#[test]
fn token_new_test() {
    let _ = Token::new_numi32(10);
    let _ = Token::new_numf32(12.04);
    let _ = Token::new_word(Tag::Id, "abcdefghijklmn");
}
