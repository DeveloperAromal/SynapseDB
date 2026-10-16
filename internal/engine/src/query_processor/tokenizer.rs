/// Splits SQL into tokens. A string literal becomes one token that keeps its
/// surrounding quotes, so `'John Doe'` stays a single value and the parser can
/// tell it apart from an identifier. Inside a literal `''` is an escaped quote.
pub fn tokensizer(sql: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = sql.chars().collect();
    let mut i: usize = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            '\'' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                let mut literal = String::from('\'');
                let mut closed = false;
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\'' {
                        if i + 1 < chars.len() && chars[i + 1] == '\'' {
                            literal.push('\'');
                            i += 2;
                            continue;
                        }
                        closed = true;
                        i += 1;
                        break;
                    }
                    literal.push(chars[i]);
                    i += 1;
                }
                if !closed {
                    return Err("Unterminated string literal".to_string());
                }
                literal.push('\'');
                tokens.push(literal);
            }
            c if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                i += 1;
            }
            ',' | '(' | ')' | ';' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(c.to_string());
                i += 1;
            }
            '!' | '>' | '<' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(format!("{}=", c));
                    i += 2;
                } else {
                    tokens.push(c.to_string());
                    i += 1;
                }
            }
            '=' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push("=".to_string());
                i += 1;
            }
            _ => {
                current.push(c);
                i += 1;
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens
        .into_iter()
        .map(|t| {
            if t.starts_with('\'') {
                return t;
            }
            let upper = t.to_uppercase();
            match upper.as_str() {
                "SELECT" | "FROM" | "WHERE" | "INSERT" | "INTO" | "VALUES" | "CREATE" | "TABLE" => upper,
                _ => t,
            }
        })
        .collect())
}
