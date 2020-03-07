#[cfg(test)]
mod token {
    use super::super::super::lexer::{Token, Tag};

    #[test]
    fn new_test() {
        let _ = Token::new(Tag.Num);
    }
}
