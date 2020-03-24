use super::Parser;
use super::super::lexer::Lexer;
use super::super::mtype::Type;
use super::super::token::{Token, Tag};
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

    /// decl: 定義文
    /// 記号表への登録を行う
    fn decl(&mut self) {
        let type_t = Self::except(self, Tag::Type);
        let id_t = Self::except(self, Tag::Id);
        if let Ok(Token::Word { tag: _, lexeme }) = type_t {
            let ty = match &lexeme[..] {
                "i32" => Type::new_i32(),
                "f32" => Type::new_f32(),
                "char" => Type::new_char(),
                _ => Type::new_i32()    // ここに来ることは絶対無いけど...
            };
            if let Ok(Token::Word { tag: _, lexeme }) = id_t {
                self.symboltable.add(Symbol::new(lexeme, ty));
            } else {
                panic!("decl: excepted => <Id>");
            }
        } else {
            panic!("decl: excepted => <Type>");
        }
    }

    /// factor: 構文の最小単位
    /// 記号表をチェックして登録済みのIDなら出力
    fn factor(&mut self) {
        if let Ok(Token::Word { tag: _, lexeme }) = Self::except(self, Tag::Id) {
            match self.symboltable.search(lexeme.clone()) {
                Some(symbol) => print!("{}:{}", symbol.lexeme, symbol.ty),
                _ =>            panic!("factor: undefined symbol => {}", lexeme)
            }
        } else {
            panic!("factor: excepted => <Id>");
        }
    }

    /// 次に続くトークンのタグを指定して取り出す
    /// 要求と異なる場合はErrを返す
    ///
    /// # params
    /// etag: Tag => 要求するタグ
    ///
    /// # return
    /// Result<Token, ()>
    fn except(&mut self, etag: Tag) -> Result<Token, ()> {
        if let Some(token) = self.lexer.scan() {
            match token {
                Token::Word { tag, lexeme } if tag == etag => {
                    Ok(Token::new_word(tag, &lexeme))
                },
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefParser;
    use super::super::super::lexer::Lexer;
    use super::super::super::token::{Tag, Token};

    #[test]
    fn defparser_simple_test() {
        let program = "{ i32 x; i32 y; { f32 x; x; y; } x; y; }".to_string();
        let lexer = Lexer::new(program);
        let _ = DefParser::new(lexer);
    }

    #[test]
    fn def_parser_decl_factor_test() {
        let mut lexer = Lexer::new("i32 a f32 b char c a b c".to_string());
        lexer.reserve(Token::new_word(Tag::Type, "i32"));
        lexer.reserve(Token::new_word(Tag::Type, "f32"));
        lexer.reserve(Token::new_word(Tag::Type, "char"));

        let mut parser = DefParser::new(lexer);
        parser.decl();
        parser.decl();
        parser.decl();
        parser.factor();
        parser.factor();
        parser.factor();
    }
}