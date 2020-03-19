use super::Parser;
use super::super::lexer::Lexer;
use super::super::token::Token;
use super::super::symbol::{Symbol, SymbolTable};

/// 定義と使用からなるプログラムをパースする
///
/// # members
/// - lexer: Lexer => 字句解析器
/// - symboltable: SymbolTable => 記号表
struct DefParser {
    lexer: Lexer,
    symboltable: SymbolTable
}

impl Parser for DefParser {}

impl DefParser {
    /// DefParserを生成して返す
    ///
    /// # params
    /// - lexer: Lexer => パースしたいプログラムで初期化された字句解析器
    ///
    /// # returns
    /// DefParser
    fn new(lexer: Lexer) -> DefParser {
        DefParser { lexer, symboltable: SymbolTable::new() }
    }

    /// DefParserを生成して返す(+記号表セット)
    ///
    /// # params
    /// - lexer: Lexer => パースしたいプログラムで初期化された字句解析器
    /// - symboltable: SymbolTable => セットする記号表
    ///
    /// # returns
    /// DefParser
    pub fn new_with_symboltable(lexer: Lexer, symboltable: SymbolTable) -> DefParser {
        DefParser { lexer, symboltable }
    }

    /// factor: 構文の最小単位
    /// 記号表をチェックして登録済みのIDなら出力
    fn factor(&mut self) {
        if let Some(token) = self.lexer.scan() {
            if let Token::Word { tag: _, lexeme } = token {
                match self.symboltable.search(lexeme.clone()) {
                    Some(symbol) => {
                        let Symbol { lexeme: _, ty } = symbol;
                        println!("{}:{}", lexeme, ty);
                    },
                    None => panic!("undefined symbol => {}", lexeme)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefParser;
    use super::super::super::lexer::Lexer;

    #[test]
    fn defparser_simple_test() {
        let program = "{ i32 x; i32 y; { f32 x; x; y; } x; y; }".to_string();
        let lexer = Lexer::new(program);
        let _ = DefParser::new(lexer);
    }
}