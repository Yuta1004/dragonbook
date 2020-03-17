use super::token::Tag;

/// 型を管理する列挙体
///
/// # members
pub enum Type {
    I32 { tag: Tag, size: usize },
    F32 { tag: Tag, size: usize },
    Char { tag: Tag, size: usize },
}

impl Type {
    /// Type::I32構造体を生成して返す
    ///
    /// # returns
    /// - Type
    pub fn new_i32() -> Type {
        Type::I32 { tag: Tag::I32, size: 4 }
    }

    /// Type::F32構造体を生成して返す
    ///
    /// # returns
    /// - Type
    pub fn new_f32() -> Type {
        Type::F32 { tag: Tag::F32, size: 4 }
    }

    /// Type::Char構造体を生成して返す
    ///
    /// # returns
    /// - Type
    pub fn new_char() -> Type {
        Type::Char { tag: Tag::Char, size: 1 }
    }
}

#[cfg(test)]
#[test]
fn type_new_test() {
    let _ = Type::new_i32();
    let _ = Type::new_f32();
    let _ = Type::new_char();
}