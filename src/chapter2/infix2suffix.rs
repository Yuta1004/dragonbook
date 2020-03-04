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


    /// 現在読んでいる文字が数字ならその値を出力する
    fn term(&mut self) {
        let lookahead = Self::get_lookahead(self);
        if '0' <= lookahead && lookahead <= '9' {
            self.lookidx += 1;
            print!("{}", lookahead);
        } else {
            Self::error(self, "expected \"term\"");
        }
    }

    /// 現在読んでいる文字との比較を行ってその結果を返す
    /// 文字が存在しない場合はpanic
    fn expect(&mut self, c: char) -> bool {
        let lookahead = match self.expr.chars().nth(self.lookidx as usize) {
            Some(c)  => c,
            None => { Self::error(self, ""); 'x' }
        };
        if lookahead == c {
            self.lookidx += 1;
            true
        } else {
            false
        }
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

