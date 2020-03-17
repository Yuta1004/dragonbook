use std::clone::Clone;
use std::str::FromStr;

/// Tag
///
/// # derive
/// - Clone
/// - PartialEQ
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

/// Token
///
/// # members
/// - NumI32 { num: i32 }   => 整数
/// - NumF32 { num: f32 }   => 小数
/// - Word { tag: Tag, lexeme: String } => 語
///
/// # derive
/// - Clone
#[derive(Clone)]
pub enum Token {
    NumI32 { num: i32 },
    NumF32 { num: f32 },
    Word { tag: Tag, lexeme: String }
}

impl Token {
    /// Token::NumI32構造体を生成して返す
    ///
    /// # params
    /// - num: i32 => 初期化する整数
    ///
    /// # returns
    /// - Token
    pub fn new_numi32(num: i32) -> Token {
        Token::NumI32 { num }
    }

    /// Token::NumF32構造体を生成して返す
    /// # params
    /// - num: f32 => 初期化する小数a
    ///
    /// # returns
    /// - Token
    pub fn new_numf32(num: f32) -> Token {
        Token::NumF32 { num }
    }

    /// Token::Word構造体を生成して返す
    ///
    /// # params
    /// - tag: Tag => 関連付けるTag
    /// - lexeme: &str => 語
    ///
    /// # returns
    /// - Token
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
