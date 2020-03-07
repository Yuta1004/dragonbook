use super::super::token::Token;
use super::super::super::lexer::Tag;

#[test]
fn new_test() {
    let _ = Token::new_num(10);
    let _ = Token::new_word(Tag::Id, "abcdefghijklmn".to_string());
}
