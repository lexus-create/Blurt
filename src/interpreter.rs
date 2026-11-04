use crate::parser::{Statement, value}; // CHANGE 1
use std::collections::HashMap;

pub fn interpret(statements: Vec<Statement>) {
    let mut vari: HashMap<String, value> = HashMap::new(); // CHANGE 2

    for stmt in statements {
        match stmt {
            Statement::Blurt(text) => {
                let mut text = text.clone();

                while let Some(open) = text.find('{') {
                    let close = open + text[open..].find('}').unwrap();

                    let key = text[open + 1..close].to_string(); // "name"
                    let mut replaced = false;

                    for (name, val) in &vari { // CHANGE 3
                        if *name == key {
                            // CHANGE 4: turn whichever kind of value it is into text
                            let as_text = match val {
                                value::stri(s) => s.clone(),
                                value::num(n) => n.to_string(),
                            };
                            replaced = true;
                            text.replace_range(open..=close, &as_text); // CHANGE 5
                            break;
                        }
                    }
                    if !replaced {
                        break;
                    }
                }

                println!("{}", text);
            }
            Statement::Let(name, val) => { // CHANGE 6
                vari.insert(name, val);    // CHANGE 7
            }
        }
    }
}
