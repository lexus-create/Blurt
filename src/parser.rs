pub struct ParseResult {
    pub print: bool,
    pub text: String,
}

pub fn parse(tokens: &[&str]) -> Vec<ParseResult> {
    let mut found_blurt = false;
    let mut in_quotes = false;
    let mut printed = String::new();
    let mut results = Vec::new();
    let mut added: Vec<i32> = Vec::new();

    for token in 0..tokens.len() {
        // whitespace only matters inside quotes
        if !in_quotes && matches!(tokens[token], " " | "\n" | "\r") {
            continue;
        }

        if tokens[token] == "blurt" && !in_quotes {
            found_blurt = true;
            continue;
        }

        if !found_blurt {
            eprintln!("error: expected `blurt`, found `{}`", tokens[token]);
            return results;
        }

        if tokens[token] == "\"" {
            in_quotes = !in_quotes;
            continue;
        }

        if in_quotes {
            printed.push_str(tokens[token]);
        } else if tokens[token] == ";" {
            results.push(ParseResult {
                print: true,
                text: std::mem::take(&mut printed), // moves the text out, leaves "" behind
            });
            found_blurt = false; // ready for the next statement
        }
    }

    results
}