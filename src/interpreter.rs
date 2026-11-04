use crate::parser::ParseResult;

pub fn interpret(todo: Vec<ParseResult>) {
    for parsed in todo.iter() {
        if parsed.print == true {
            println!("{}", parsed.text)
        }
    }
}