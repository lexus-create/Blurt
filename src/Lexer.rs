pub fn tokenize(line: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut start = 0;

    for (i, c) in line.char_indices() {
        if matches!(c, ',' | ';' | '+' | ':' | ' ' | '=' | '"' | '\\' | '\n' | '\r' | '{' | '}' | '#' ) {
            
            if start < i {
                tokens.push(&line[start..i]);
            }
            

            tokens.push(&line[i..i + c.len_utf8()]);
  
            start = i + c.len_utf8();
        }
    }

   
    if start < line.len() {
        tokens.push(&line[start..]);
    }

    tokens
}
