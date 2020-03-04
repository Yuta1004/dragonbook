use std::env;
use std::process::exit;
use dragonbook::chapter2::infix2suffix::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[usage] cargo run 1-2+0-4");
        exit(1);
    }
    let expr = args[1].to_string();

    let mut parser = Parser::new(expr);
    parser.parse();
    println!();
}
