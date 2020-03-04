use dragonbook::chapter2::infix2suffix::Parser;

fn main() {
    let mut parser = Parser::new("1-2+0-4".to_string());
    parser.parse();
    println!();
}
