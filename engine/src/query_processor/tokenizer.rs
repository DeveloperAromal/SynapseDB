pub fn tokensizer(sql: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = sql.chars().collect();
    let mut i: usize = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\n' | '\t' => {
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

    tokens
        .into_iter()
        .map(|t| {
            let upper = t.to_uppercase();
            match upper.as_str() {
                "SELECT" | "FROM" | "WHERE" | "INSERT" | "INTO" | "VALUES" | "CREATE" | "TABLE" => upper,
                _ => t,
            }
        })
        .collect()
}
