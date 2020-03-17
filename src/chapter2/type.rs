use super::token::Tag;

/// 型を管理する列挙体
///
/// # members
pub enum Type {
    Int { tag: Tag, size: usize },
    Float { tag: Tag, size: usize },
    Char { tag: Tag, size: usize },
    Str { tag: Tag, size: usize }
}