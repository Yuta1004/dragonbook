pub struct Parser {
    expr: String,
    lookidx: i32,
}

impl Parser {
    pub fn new(expr: String) -> Parser {
        Parser{ expr, lookidx: 0 }
    }
}

#[test]
fn new_test() {
    let _ = Parser::new("1+1".to_string());
}
