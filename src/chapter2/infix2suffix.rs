pub struct Parser {
    expr: String,
    lookidx: i32,
}

impl Parser {
    /// Parser構造体を生成して返す
    ///
    /// # Params
    /// - expr(String) : 中置式
    ///
    /// # Return
    /// Parser
    pub fn new(expr: String) -> Parser {
        Parser{ expr, lookidx: 0 }
    }

    /// エラーを吐いてpanicを投げる
    ///
    /// # Params
    /// - msg(&str) : メッセージ
    fn error(msg: &str) {
        panic!("[ERROR] {}", msg);
    }
}

#[test]
fn new_test() {
    let _ = Parser::new("1+1".to_string());
}
