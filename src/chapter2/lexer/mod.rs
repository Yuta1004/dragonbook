#[cfg(test)]
mod tests;

// Token
pub struct Token {
    tag: i32,
}

impl Token {
    pub fn new(tag: i32) -> Token {
        Token { tag }
    }
}

// Tag
pub enum Tag {
    Plus,
    Sub,
    Mul,
    Div,
    Num,
    Id,
    True,
    False
}
