mod token;

use std::collections::HashMap;

use token::{Tag, Token};

pub struct Lexer {
    line: usize,
    nowon: usize,
    program: Vec<char>,
    match_table: HashMap<String, Token>
}

impl Lexer {
    pub fn new(program: String) -> Lexer {
        let mut lexer = Lexer {
            line: 0,
            nowon: 0,
            program: program.chars().collect::<Vec<char>>(),
            match_table: HashMap::new()
        };
        lexer.reserve(Token::new_word(Tag::True, "true".to_string()));
        lexer.reserve(Token::new_word(Tag::False, "false".to_string()));
        lexer
    }

    pub fn scan(&mut self) -> Token {
        let (size, line) = skip_space(&self.program[self.nowon..]);
        self.nowon += size;
        self.line += line;

        let target = &self.program[self.nowon..];
        match target[0] {
            '0'..='9' => {
                let (num, size) = consume_num(target);
                self.nowon += size;
                if num - (num as i32) as f32 > 0.0 {
                    Token::new_numf32(num)
                } else {
                    Token::new_numi32(num as i32)
                }
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let (word, size) = consume_word(target);
                self.nowon += size;
                match self.match_table.get(&word) {
                    Some(t) => t.clone(),
                    None => {
                        let nt = Token::new_word(Tag::Id, word);
                        Self::reserve(self, nt.clone());
                        nt
                    }
                }
            },
            c => panic!("[FAILED] error at line:{} => {}", self.line, c)
        }
    }

    fn reserve(&mut self, token: Token) {
        match token.clone() {
            Token::Word { tag: _, lexeme } => {
                self.match_table.insert(lexeme.clone(), token);
            },
            _ => {}
        };
    }
}

fn skip_space(target_vec: &[char]) -> (usize, usize) {
    let mut size = 0;
    let mut line = 0;
    for c in target_vec {
        match c {
            ' ' | '\t' => size += 1,
            '\n' => { line += 1; size += 1 },
            _ => break
        }
    }
    (size, line)
}

fn consume_num(target_vec: &[char]) -> (f32, usize) {
    let mut num_str = String::new();
    for c in target_vec {
        match c {
            '0'..='9' | '.' => num_str.push(*c),
            _ => break
        }
    }
    (num_str.parse::<f32>().unwrap(), num_str.chars().count())
}

fn consume_word(target_vec: &[char]) -> (String, usize) {
    let mut word = String::new();
    for c in target_vec {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => word.push(*c),
            _ => break
        }
    }
    let size = word.chars().count();
    (word, size)
}

#[test]
fn lexer_simple_test() {
    // word
    let mut lexer = Lexer::new("gochiusa  cocoa".to_string());
    match lexer.scan() {
        Token::Word { tag: _, lexeme } if lexeme == "gochiusa" => {},
        _ => panic!("test failed at [lexer_simple_test::word]")
    }

    // num(i32)
    lexer = Lexer::new("1204".to_string());
    match lexer.scan() {
        Token::NumI32 { num } if num == 1204 => {},
        _ => panic!("test failed at [lexer_simple_test::num(i32)")
    }

    // num(f32)
    lexer = Lexer::new("1004.1204014".to_string());
    match lexer.scan() {
        Token::NumF32 { num } if num == 1004.1204014 => {},
        _ => panic!("test failed at [lexer_simple_test::num(f32)]")
    }
}
