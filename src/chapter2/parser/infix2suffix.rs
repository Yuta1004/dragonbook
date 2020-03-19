use super::Parser;

pub struct Infix2Suffix {
    expr: String,
    lookidx: i32,
}

impl Parser for Infix2Suffix {}

impl Infix2Suffix {
    /// Parser構造体を生成して返す
    ///
    /// # Params
    /// - expr(String) : 中置式
    ///
    /// # Return
    /// Parser
    pub fn new(expr: String) -> Infix2Suffix {
        Infix2Suffix { expr, lookidx: 0 }
    }

    /// expr -> term rest
    /// rest -> + term {print!("+")} rest
    ///       | - term {print!("-")} rest
    ///       | ε
    pub fn parse(&mut self) {
        Self::term(self);
        loop {
            if Self::expect(self, '+') {
                Self::term(self);
                print!("+");
                continue
            } else if Self::expect(self, '-') {
                Self::term(self);
                print!("-");
                continue
            }
            break
        }
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
    fn expect(&mut self, c: char) -> bool {
        let lookahead = match self.expr.chars().nth(self.lookidx as usize) {
            Some(c)  => c,
            None => '_'
        };
        if lookahead != '_' && lookahead == c {
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
        let space_len = 11+msg.len()+self.lookidx as usize;
        let mut space = String::with_capacity(space_len);
        for _ in 0..space_len {
            space.push(' ');
        }
        panic!("\n[ERROR] {} : {}\n{}^at here\n", msg, self.expr, space);
    }
}

#[cfg(test)]
mod tests {
    use super::super::infix2suffix::Infix2Suffix;

    #[test]
    fn new_test() {
        let _ = Infix2Suffix::new("1+1".to_string());
    }

    #[test]
    fn parse_test() {
        let mut parser = Infix2Suffix::new("9+5-2".to_string());
        parser.parse();
    }

    #[test]
    fn term_test() {
        let mut parser = Infix2Suffix::new("2+2".to_string());
        parser.term();
    }

    #[test]
    fn expect_test() {
        let mut parser = Infix2Suffix::new("abcdefg".to_string());
        assert!(parser.expect('a'));
        assert!(!parser.expect('A'));
    }

    #[test]
    fn lookahead_test() {
        let parser = Infix2Suffix::new("abcdefg".to_string());
        assert_eq!('a', parser.get_lookahead());
    }

    #[test]
    #[should_panic]
    fn error_test() {
        let parser = Infix2Suffix::new("abcdefg".to_string());
        parser.error("PANIC");
    }
}