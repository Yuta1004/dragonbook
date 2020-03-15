use super::super::token::{Tag, Token};

#[test]
fn new_test() {
    let _ = Token::new_numi32(10);
    let _ = Token::new_word(Tag::Id, "abcdefghijklmn".to_string());
}
