use std::fmt;

/// 型を管理する列挙体
///
/// # members
/// - I32 { tag: Tag, size: usize } => 整数型(32)
/// - F32 { tag: Tag, size: usize } => 小数型(32)
/// - Char { tag: Tag, size: usize } => 文字型(8)
///
/// # derive
/// - PartialEq
/// - Clone
#[derive(PartialEq, Clone)]
pub enum Type {
    I32 (usize),
    F32 (usize),
    Char (usize),
}

impl Type {
    /// Type::I32構造体を生成して返す
    ///
    /// # returns
    /// - Type
    pub fn new_i32() -> Type {
        Type::I32(4)
    }

    /// Type::F32構造体を生成して返す
    ///
    /// # returns
    /// - Type
    pub fn new_f32() -> Type {
        Type::F32(4)
    }

    /// Type::Char構造体を生成して返す
    ///
    /// # returns
    /// - Type
    pub fn new_char() -> Type {
        Type::Char(1)
    }
}

/// Displayトレイト
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 (_) => write!(f, "i32"),
            Type::F32 (_) => write!(f, "f32"),
            Type::Char (_) => write!(f, "char")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn type_new_test() {
        let _ = Type::new_i32();
        let _ = Type::new_f32();
        let _ = Type::new_char();
    }
}