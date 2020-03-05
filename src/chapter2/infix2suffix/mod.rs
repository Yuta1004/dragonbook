mod tests;

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
