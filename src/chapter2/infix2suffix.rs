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

    /// 現在読んでいる文字との比較を行ってその結果を返す
    /// 文字が存在しない場合はpanic
    fn expect(&self, c: char) -> bool {
        let lookahead = match(self.expr.chars().nth(self.lookidx as usize)) {
            Some(c)  => c,
            None => { Self::error(self, ""); 'x' }
        };
        lookahead == c
    }

    /// 現在対象となっている文字を返す
    /// 存在しない場合はpanic
    fn get_lookahead(&self) -> char {
        match self.expr.chars().nth(self.lookidx as usize) {
            Some(c)  => c,
            None => { Self::error(self, ""); 'x' }
        }
    }

    /// エラーを吐いてpanicを投げる
    ///
    /// # Params
    /// - msg(&str) : メッセージ
    fn error(&self, msg: &str) {
        panic!("[ERROR] {} : at {}", msg, self.lookidx);
    }
}

#[test]
fn new_test() {
    let _ = Parser::new("1+1".to_string());
}
