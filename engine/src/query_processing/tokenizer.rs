pub fn tokensizer(sql: &str) -> Vec<String> {
    let mut tokens: Vec::new();
    let current = String::new();

    for c in sql.chars() {
        match c {
            ' ' | '\n' | '\t' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear()
                }
            }

            ',' | '(' | ')' | ';' | '=' | '>' | '<' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear()
                }
                tokens.push(c.to_string())
            }

            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current.clone());
    }

    tokens
        .into_iter()
        .map(|t| {
            let upper = t.to_uppercase();
            match upper.as_str() {
                "SELECT" | "FROM" | "WHERE" | "INSERT" | "INTO" | "VALUES" | "CREATE" | "TABLE" => {
                    upper
                }
                _ => t,
            }
        })
        .collect()
}
