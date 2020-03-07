#[cfg(test)]
mod token {
    use super::super::super::lexer::Token;

    #[test]
    fn new_test() {
        let _ = Token::new(1);
    }
}
