use std::collections::HashMap;

use super::mtype::Type;

/// 記号表で管理する1単位を表す
///
/// # members
/// - lexeme: String => 語
/// - ty: Type => 型
///
/// # derive
/// - PartialEq
/// - Clone
#[derive(PartialEq, Clone)]
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
    pub fn new() -> SymbolTable {
        SymbolTable { prev: Box::new(None), table: HashMap::new( )}
    }

    /// 親を持った記号表を生成する
    ///
    /// # params
    /// prev: Box<Option<SymbolTable>> => 親となる記号表
    ///
    /// # return
    /// - Box<Option<SymbolTable>>
    pub fn new_with_table(prev: SymbolTable) -> SymbolTable {
        SymbolTable { prev: Box::new(Some(prev)), table: HashMap::new() }
    }

    /// 記号表に要素を追加する
    ///
    /// # params
    /// - symbol: Symbol => 追加する記号要素
    pub fn add(&mut self, symbol: Symbol) {
        self.table.insert(symbol.lexeme.clone(), symbol);
    }

    /// 記号表から要素を検索する
    ///
    /// # params
    /// - target: String => 要素名
    ///
    /// # returns
    /// Symbol
    pub fn search(&self, target: String) -> Option<Symbol>{
        Self::search_table(&target, self)
    }

    fn search_table(target: &String, symboltable: &SymbolTable) -> Option<Symbol> {
        match symboltable.table.get(target) {
            Some(symbol) => Some(symbol.clone()),
            None => {
                match *(symboltable.prev) {
                    Some(ref table) => Self::search_table(target, table),
                    None => None
                }
            }
        }
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
    fn symboltable_simple_test() {
        let table_a = SymbolTable::new();
        let mut table_b  = SymbolTable::new_with_table(table_a);

        table_b.add(
            Symbol::new("a".to_string(), Type::new_i32())
        );
        table_b.add(
            Symbol::new("b".to_string(), Type::new_f32())
        );

        for tag in vec!["a", "b"] {
            match table_b.search(tag.to_string()) {
                Some(Symbol { lexeme, ty: _ }) if lexeme == tag => {},
                _ => panic!("test failed at [symboltable_simple_test]")
            }
        }
    }
}