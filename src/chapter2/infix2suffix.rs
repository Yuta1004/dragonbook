pub struct Parser {
    expr: String
}

impl Parser {
    pub fn new(expr: String) -> Parser {
        Parser{ expr }
    }
}

#[test]
fn new_test() {
    let _ = Parser::new("1+1".to_string());
}
