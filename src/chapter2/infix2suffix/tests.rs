#[test]
fn new_test() {
    let _ = Parser::new("1+1".to_string());
}

#[test]
fn parse_test() {
    let mut parser = Parser::new("9+5-2".to_string());
    parser.parse();
}

#[test]
fn term_test() {
    let mut parser = Parser::new("2+2".to_string());
    parser.term();
}

#[test]
fn expect_test() {
    let mut parser = Parser::new("abcdefg".to_string());
    assert!(parser.expect('a'));
    assert!(!parser.expect('A'));
}

#[test]
fn lookahead_test() {
    let parser = Parser::new("abcdefg".to_string());
    assert_eq!('a', parser.get_lookahead());
}

#[test]
#[should_panic]
fn error_test() {
    let parser = Parser::new("abcdefg".to_string());
    parser.error("PANIC");
}
