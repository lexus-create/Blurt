mod lexer;
mod parser;
mod interpreter;

use std::fs;

fn main() {
    let code = fs::read_to_string("script.blurt").expect("couldn't read file");
    let tokens = lexer::tokenize(&code);
    println!("{:?}", tokens);
    interpreter::interpret(parser::parse(&tokens));
}