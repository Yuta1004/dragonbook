mod token;

use std::collections::HashMap;

use token::{Tag, Token};

pub struct Lexer {
    line: i32,
    nowon: i32,
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

    fn reserve(&mut self, token: Token) {
        match token.clone() {
            Token::Word { tag: _, lexeme } => {
                self.match_table.insert(lexeme.clone(), token);
            },
            _ => {}
        };
    }
}

fn skip_space(target_vec: &[char]) -> i32 {
    let mut size = 0;
    for c in target_vec {
        match c {
            ' ' => size += 1,
            _ => break
        }
    }
    size
}

fn consume_num(target_vec: &[char]) -> (i32, i32) {
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

fn consume_word(target_vec: &[char]) {
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
