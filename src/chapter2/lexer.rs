mod token;

use std::collections::HashMap;

use token::{Tag, Token};

pub struct Lexer {
    line: i32,
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
        let space_num = skip_space(&self.program[self.nowon..]);
        self.nowon += space_num;

        let target = &self.program[self.nowon..];
        match target[0] {
            '0'..='9' => {
                let (num, size) = consume_num(target);
                self.nowon += size;
                Token::new_num(num)
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let (word, size) = consume_word(target);
                self.nowon += size;
                Token::new_word(Tag::Id, word)
            },
            c => panic!("i can't translate this word => {}", c)
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

fn skip_space(target_vec: &[char]) -> usize {
    let mut size = 0;
    for c in target_vec {
        match c {
            ' ' => size += 1,
            _ => break
        }
    }
    size
}

fn consume_num(target_vec: &[char]) -> (i32, usize) {
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

fn consume_word(target_vec: &[char]) -> (String, usize) {
    let mut word = String::new();
    let mut size = 0;
    for c in target_vec {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                word.push(*c);
                size += 1;
            },
            _ => break
        }
    }
    (word, size)
}

#[test]
fn lexer_easy_test() {
    let mut lexer = Lexer::new("gochiusa.com".to_string());
    match lexer.scan() {
        Token::Num { tag: _, num } => println!("Num: {}", num),
        Token::Word { tag: _, lexeme } => println!("Word: {}", lexeme),
    }
}
