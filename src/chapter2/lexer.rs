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
        let mut program = program;
        program.push('@');
        let mut lexer = Lexer {
            line: 0,
            nowon: 0,
            program: program.chars().collect::<Vec<char>>(),
            match_table: HashMap::new()
        };
        lexer.reserve(Token::new_word(Tag::True, "true"));
        lexer.reserve(Token::new_word(Tag::False, "false"));
        lexer.reserve(Token::new_word(Tag::Equal, "=="));
        lexer.reserve(Token::new_word(Tag::NotEqual, "!="));
        lexer.reserve(Token::new_word(Tag::UpperThanL, "<"));
        lexer.reserve(Token::new_word(Tag::UpperThanR, ">"));
        lexer.reserve(Token::new_word(Tag::UpperEqThanL, "<="));
        lexer.reserve(Token::new_word(Tag::UpperEqThanR, ">="));
        lexer
    }

    pub fn scan(&mut self) -> Option<Token> {
        if self.program.len() <= self.nowon {
            return None
        }
        let (size, line) = skip_space(&self.program[self.nowon..]);
        self.nowon += size;
        self.line += line;

        let target = &self.program[self.nowon..];
        match target[0] {
            '0'..='9' => {
                let (num, size) = consume_num(target);
                self.nowon += size;
                if num - (num as i32) as f32 > 0.0 {
                    Some(Token::new_numf32(num))
                } else {
                    Some(Token::new_numi32(num as i32))
                }
            },
            'A'..='~' | '!'..='/' | ':'..='?' => {
                let (word, size) = consume_word(target);
                self.nowon += size;
                match self.match_table.get(&word) {
                    Some(t) => Some(t.clone()),
                    None => {
                        let nt = Token::new_word(Tag::Id, &word);
                        Self::reserve(self, nt.clone());
                        Some(nt)
                    }
                }
            },
            '@' => { self.nowon += 1; None }
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
            'A'..='~' | '!'..='/' | ':'..='?' => word.push(*c),
            _ => break
        }
    }
    let size = word.chars().count();
    (word, size)
}

#[test]
fn lexer_simple_test() {
    let program =
    "\
abcde efghj klmno pqrst uvwxy z
123 456 789 012
1.23456789 0.00123456
< > <= >= != == true false
    ".to_string();

    let mut lexer = Lexer::new(program);
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
