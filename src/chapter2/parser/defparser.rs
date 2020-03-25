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

    /// block: ブロック
    fn block(&mut self) -> Result<(), String> {
        for cnt in 0..=1 {
            let block_s = Self::except(self, Tag::Symbol)?;
            if let Token::Word { tag: _, lexeme } = block_s {
                if cnt == 0 && lexeme == "{" {
                    self.symboltable = SymbolTable::new_with_table(self.symboltable.clone());
                    Self::stmts(self);
                } else if cnt == 1 && lexeme == "}" {
                    self.symboltable = self.symboltable.clone().release().unwrap();
                } else {
                    panic!("block undefined/unclosed!!");
                }
            }
        }
        Ok(())
    }

    /// stmts: 文の集合
    ///
    /// # returns
    /// Result<(), String>
    fn stmts(&mut self) {
        loop {
            match Self::stmt(self) {
                Ok(_) => continue,
                Err(msg) if msg != "eof" => panic!(msg),
                _ => break
            }
        }
    }

    fn stmt(&mut self) -> Result<(), String> {
        let token = self.lexer.scan().ok_or("eof".to_string())?;
        if let Token::Word { tag, lexeme: _ } = token.clone() {
            match tag {
                Tag::Type => {
                    let id_t = Self::except(self, Tag::Id)?;
                    Self::decl(self, token, id_t);
                },
                Tag::Id => Self::factor(self, token),
                _ => {}
            }
        }
        Ok(())
    }

    /// decl: 定義文
    /// 記号表への登録を行う
    ///
    /// # params
    /// - ty_t: Token => Tag::TypeであるToken
    /// - id_t: Token => Tag::IdであるToken
    fn decl(&mut self, ty_t: Token, id_t: Token) {
        if let Token::Word { tag: _, lexeme } = ty_t {
            let ty = match &lexeme[..] {
                "i32" => Type::new_i32(),
                "f32" => Type::new_f32(),
                "char" => Type::new_char(),
                _ => Type::new_i32()    // ここに来ることは絶対無いけど...
            };
            if let Token::Word { tag: _, lexeme } = id_t {
                self.symboltable.add(Symbol::new(lexeme, ty));
            }
        }
    }

    /// factor: 構文の最小単位
    /// 記号表をチェックして登録済みのIDなら出力
    ///
    /// # params
    /// - id_t: Token => Tag::IdであるToken
    fn factor(&mut self, id_t: Token) {
        if let Token::Word { tag: _, lexeme } = id_t {
            match self.symboltable.search(lexeme.clone()) {
                Some(symbol) => print!("{}:{}", symbol.lexeme, symbol.ty),
                _ =>            panic!("factor: undefined symbol => {}", lexeme)
            }
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
    fn except(&mut self, etag: Tag) -> Result<Token, String> {
        if let Some(token) = self.lexer.scan() {
            match token {
                Token::Word { tag, lexeme } if tag == etag => {
                    Ok(Token::new_word(tag, &lexeme))
                },
                _ => Err(format!("excepted => <{}>", etag))
            }
        } else {
            Err(format!("notice => eof"))
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
        let mut lexer = Lexer::new("{ i32 a f32 b char c a b c }".to_string());
        lexer.reserve(Token::new_word(Tag::Type, "i32"));
        lexer.reserve(Token::new_word(Tag::Type, "f32"));
        lexer.reserve(Token::new_word(Tag::Type, "char"));
        let mut parser = DefParser::new(lexer);
        let _ = parser.block();
    }
}