#[cfg(test)]
mod tests;

pub struct Token {
    tag: i32,
}

impl Token {
    pub fn new(tag: i32) -> Token {
        Token { tag }
    }
}
