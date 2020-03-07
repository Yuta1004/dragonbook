#[cfg(test)]
mod tests;

// Token
pub struct Token {
    tag: Tag,
}

impl Token {
    pub fn new(tag: Tag) -> Token {
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
