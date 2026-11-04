pub enum Statement {
    Blurt(String),      //text
    Let(String, value), //name, value
}

pub enum value {
    stri(String),
    num(i32),
}

pub fn parse(tokens: &[&str]) -> Vec<Statement> {
    let mut found_blurt = false;
    let mut in_quotes = false;
    let mut printed = String::new();
    let mut results = Vec::new();
    let mut added: Vec<i32> = Vec::new();
    let mut doingadding = false;
    let mut initiatedvar = false;
    let mut var_name = String::new();
    let mut var_valuestr = String::new();
    let mut var_valuenum = 5;
    let mut found_name = false;
    let mut equalled = false;
    let mut discoveredplus = false;
    let mut discoveredequalsearly = false;
    let mut discoveredequalsprime = false;
    let mut numstrswitch = false;
    let mut incomment = false;

    for token in 0..tokens.len() {
        if tokens[token] == "#" && !incomment && !in_quotes {
            incomment = true;
            continue;
        }

        if incomment && !in_quotes && matches!(tokens[token], "\n") {
            incomment = false;
        }

        if incomment {
            continue;
        }

        if !in_quotes && matches!(tokens[token], " " | "\n" | "\r") {
            continue;
        }

        if tokens[token] == "blurt" && !in_quotes && !initiatedvar {
            found_blurt = true;
            continue;
        } else if initiatedvar == true && tokens[token] == "blurt" {
            eprintln!("Cannot use blurt as variable name");
            return results;
        }

        if tokens[token] == "let" && !in_quotes {
            initiatedvar = true;
            continue;
        }

        if initiatedvar == true {
            var_name = tokens[token].to_string();
            initiatedvar = false;
            found_name = true;
            continue;
        }

        if found_name == true {
            if tokens[token] == "+" {
                discoveredplus = true;
                continue;
            } else if tokens[token] == "=" {
                found_name = false;
                discoveredequalsearly = true;
                continue;
            }
        }

        if discoveredequalsearly == true {
            if tokens[token] == "\"" {
                in_quotes = !in_quotes;
                continue;
            } else if in_quotes == false {
                if let Ok(n) = tokens[token].parse::<i32>() {
                    numstrswitch = true;
                    var_valuenum = tokens[token].parse().unwrap()
                }
            }
            if in_quotes {
                var_valuestr.push_str(tokens[token]);
                continue;
            }

            if tokens[token] == ";" {
                results.push(Statement::Let(
                    std::mem::take(&mut var_name),
                    if numstrswitch == false {
                        value::stri(std::mem::take(&mut var_valuestr))
                    } else {
                        value::num(var_valuenum)
                    },
                ));
                discoveredequalsearly = false;
                numstrswitch = false;
                continue;
            }
        }

        if tokens[token] == "\"" {
            in_quotes = !in_quotes;
            continue;
        }

        if in_quotes {
            printed.push_str(tokens[token]);
        } else if tokens[token] == ";" {
            results.push(Statement::Blurt(std::mem::take(&mut printed)));
            found_blurt = false; // ready for the next statement
        }
    }

    results
}
