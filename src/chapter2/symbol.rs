use std::collections::HashMap;

use super::mtype::Type;

/// 記号表で管理する1単位を表す
///
/// # members
/// - lexeme: String => 語
/// - ty: Type => 型
#[derive(PartialEq)]
pub struct Symbol {
    lexeme: String,
    ty: Type
}

impl Symbol {
    /// Symbol構造体を生成して返す
    ///
    /// # params
    /// - lexeme: String => 語
    /// - ty: Type => 型
    ///
    /// # return
    /// - Symbol
    pub fn new(lexeme: String, ty: Type) -> Symbol {
        Symbol { lexeme, ty }
    }
}

/// 記号表
///
/// # members
/// - prev: Box<SymbolTable> => 上位に位置する記号表をもつ
/// - table: String, Symbolの照合表
pub struct SymbolTable {
    prev: Box<Option<SymbolTable>>,
    table: HashMap<String, Symbol>
}

#[cfg(test)]
#[test]
fn symbol_new_test() {
    let lexeme = "abcdefghijklmn".to_string();
    let ty = Type::new_i32();
    let _ = Symbol::new(lexeme, ty);
}