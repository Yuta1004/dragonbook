mod token;

use std::collections::HashMap;

use token::Token;

pub struct Lexer<'a> {
    line: i32,
    nowon: i32,
    program: Vec<char>,
    match_table: HashMap<String, &'a Token>
}

impl<'a> Lexer<'a> {
    pub fn new(program: String) -> Lexer<'a> {
        Lexer {
            line: 0,
            nowon: 0,
            program: program.chars().collect::<Vec<char>>(),
            match_table: HashMap::new()
        }
    }

    fn reserve(&mut self, tag: String, token: &'a Token) {
        self.match_table.insert(tag, token);
    }
}


fn skip_space(target_vec: &Vec<char>) -> i32 {
    let mut size = 0;
    for c in target_vec {
        match c {
            ' ' => size += 1,
            _ => break
        }
    }
    size
}

fn consume_num(target_vec: &Vec<char>) -> (i32, i32) {
    let mut num = 0;
    let mut size = 0;
    for c in target_vec {
        match c {
            '0'..='9' => {
                num = num*10 + (*c as i32 - '0' as i32);
                size += 1;
            },
            _ => break
        }
    }
    (num, size)
}

fn consume_word(target_vec: &Vec<char>) {
    let mut word = String::new();
    let mut size = 0;
    for c in target_vec {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => word.push(*c),
            _ => break
        }
    }
}

#[test]
fn lexer_easy_test() {
    let _ = Lexer::new("gochiusa.com".to_string());
}
