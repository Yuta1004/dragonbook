use std::env;
use std::process::exit;
use dragonbook::chapter2::parser::infix2suffix::Infix2Suffix;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[usage] cargo run 1-2+0-4");
        exit(1);
    }
    let expr = args[1].to_string();

    let mut parser = Infix2Suffix::new(expr);
    parser.parse();
    println!();
}
