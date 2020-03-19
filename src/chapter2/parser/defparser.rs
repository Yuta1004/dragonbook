use super::Parser;
use super::super::lexer::Lexer;
use super::super::symbol::{Symbol, SymbolTable};

struct DefParser {
    lexer: Lexer,
    symboltable: SymbolTable
}

impl Parser for DefParser {}

impl DefParser {
    pub fn new(lexer: Lexer) -> DefParser {
        DefParser { lexer, symboltable: SymbolTable::new() }
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