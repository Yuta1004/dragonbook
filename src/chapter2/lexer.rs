use std::collections::HashMap;

use super::token::{Tag, Token};

pub struct Lexer {
    line: usize,
    nowon: usize,
    program: Vec<char>,
    match_table: HashMap<String, Token>
}

impl Lexer {
    /// Lexer構造体を生成して返す
    ///
    /// # params
    /// - program: String => 字句解析対象文字列
    ///
    /// # returns
    /// - Lexer
    pub fn new(program: String) -> Lexer {
        Lexer {
            line: 1,
            nowon: 0,
            program: (program+"@").chars().collect::<Vec<char>>(),
            match_table: HashMap::new()
        }
    }

    /// 1字句だけ解析を行い、解析結果<Token>を返す
    ///
    /// # returns
    /// - Option<Token>
    pub fn scan(&mut self) -> Option<Token> {
        if self.program.len() <= self.nowon {
            return None
        }
        Self::skip_space(self);

        let target = &self.program[self.nowon..];
        let c = target[0]; let n = target[1];
        match c {
            // 記号
            '<' if n == '=' => {
                self.nowon += 2;
                Some(Token::new_word(Tag::UpperEqThanL, "<="))
            },
            '>' if n == '=' => {
                self.nowon += 2;
                Some(Token::new_word(Tag::UpperEqThanR, ">="))
            },
            '=' if n == '=' => {
                self.nowon += 2;
                Some(Token::new_word(Tag::Equal, "=="))
            },
            '!' if n == '=' => {
                self.nowon += 2;
                Some(Token::new_word(Tag::NotEqual, "!="))
            },
            '<' => {
                self.nowon += 1;
                Some(Token::new_word(Tag::UpperThanL, "<"))
            },
            '>' => {
                self.nowon += 1;
                Some(Token::new_word(Tag::UpperThanR, "<"))
            },
            // 数字
            '0'..='9' => {
                let num = Self::consume_num(self);
                if num - (num as i32) as f32 > 0.0 {
                    Some(Token::new_numf32(num))
                } else {
                    Some(Token::new_numi32(num as i32))
                }
            },
            // 語
            'a'..='z' | 'A'..='Z' | '_' => {
                let word = Self::consume_word(self);
                match self.match_table.get(&word) {
                    Some(t) => Some(t.clone()),
                    None => {
                        let nt = Token::new_word(Tag::Id, &word);
                        Self::reserve(self, nt.clone());
                        Some(nt)
                    }
                }
            },
            // 未定義文字
            '@' => { self.nowon += 1; None },
            _ if target[1] == '@' => { self.nowon += 1; None }
            c => panic!("[FAILED] error at line:{} => {}", self.line, c)
        }
    }

    /// 予約語など、既知の語を管理対象として追加する
    ///
    /// # params
    /// - token: Token => 追加するToken
    pub fn reserve(&mut self, token: Token) {
        match token.clone() {
            Token::Word { tag: _, lexeme } => {
                self.match_table.insert(lexeme.clone(), token);
            },
            _ => {}
        };
    }

    /// 解析中の場所から連続する空白/タブ/改行文字を読み飛ばす
    fn skip_space(&mut self) {
        let nowon = self.nowon;
        for c in &self.program[nowon..] {
            match c {
                ' ' | '\t' => self.nowon += 1,
                '\n' => { self.line += 1; self.nowon += 1 },
                _ => break
            }
        }
    }

    /// 解析中の場所から連続する数字列を読み取って、その値を返す
    ///
    /// # returns
    /// - f32
    fn consume_num(&mut self) -> f32 {
        let nowon = self.nowon;
        let mut num_str = String::new();
        for c in &self.program[nowon..] {
            match c {
                '0'..='9' | '.' => {
                    num_str.push(*c);
                    self.nowon += 1;
                },
                _ => break
            }
        }
        num_str.parse::<f32>().unwrap()
    }

    /// 解析中の場所から連続する文字列を読み取って、その値を返す
    ///
    /// # returns
    /// - String
    fn consume_word(&mut self) -> String {
        let nowon = self.nowon;
        let mut word = String::new();
        for c in &self.program[nowon..] {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    word.push(*c);
                    self.nowon += 1;
                }
                _ => break
            }
        }
        word
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::super::token::{Token, Tag};

    #[test]
    fn lexer_simple_test() {
        let program =
        "\
abcde efghj klmno pqrst uvwxy z
123 456 789 012
1.23456789 0.00123456
< > <= >= != == true false
10>=20 30<=40 1<2 3>0 abc!=def
        ".to_string();

        let mut lexer = gen_lexer(program);
        loop {
            if let Some(token) = lexer.scan() {
                match token {
                    Token::NumI32 { num } => println!("Num(i32): {}", num),
                    Token::NumF32 { num } => println!("Num(f32): {}", num),
                    Token::Word { tag: _, lexeme } => println!("Word: {}", lexeme),
                }
            } else {
                break;
            }
        }
    }

    fn gen_lexer(program: String) -> Lexer {
        let mut lexer = Lexer::new(program);
        lexer.reserve(Token::new_word(Tag::True, "true"));
        lexer.reserve(Token::new_word(Tag::False, "false"));
        lexer
    }
}