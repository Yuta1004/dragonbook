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

impl SymbolTable {
    /// rootに位置する記号表を生成して返す
    ///
    /// # returns
    /// - Box<Option<SymbolTable>>
    pub fn new() -> Box<Option<SymbolTable>> {
        Self::new_with_table(Box::new(None))
    }

    /// 親を持った記号表を生成する
    ///
    /// # params
    /// prev: Box<Option<SymbolTable>> => 親となる記号表
    ///
    /// # return
    /// - Box<Option<SymbolTable>>
    pub fn new_with_table(prev: Box<Option<SymbolTable>>) -> Box<Option<SymbolTable>> {
        Box::new(
            Some(
                SymbolTable { prev, table: HashMap::new() }
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Symbol, SymbolTable};
    use super::super::mtype::Type;

    #[test]
    fn symbol_new_test() {
        let lexeme = "abcdefghijklmn".to_string();
        let ty = Type::new_i32();
        let _ = Symbol::new(lexeme, ty);
    }

    #[test]
    fn symbol_table_new_test() {
        let root = SymbolTable::new();
        let _ = SymbolTable::new_with_table(root);
    }
}