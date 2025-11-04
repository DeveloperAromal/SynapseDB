pub mod query_processor;
pub mod storage;

use query_processor::ast::Query;
use query_processor::executor::{execute_select, execute_insert, execute_create, SelectResult};
use storage::row::DynamicField;
use query_processor::parser::Parser;

pub fn run_query(query: &str) -> String {
    match Parser::parse(query) {
        Ok(Query::Create(create)) => {
            execute_create(&create)
                .map(|_| format!("Table created: {}", create.statement.table_name))
                .unwrap_or_else(|e| format!("Error: {}", e))
        }
        Ok(Query::Insert(insert)) => {
            execute_insert(&insert)
                .map(|_| format!("Insert succeeded into {}", insert.table_name))
                .unwrap_or_else(|e| format!("Error: {}", e))
        }
        Ok(Query::Select(select)) => {
            let result = execute_select(&select);
            format_select_result(result)
        }
        #[allow(dead_code)]
        Ok(_) => "Unsupported query".to_string(),
        Err(e) => format!("Parse error: {}", e),
    }
}

use std::cmp::max;

fn format_select_result(result: SelectResult) -> String {
    if result.rows.is_empty() {
        if !result.headers.is_empty() {
            return result.headers.join(" | ");
        }
        return "Empty set".to_string();
    }

    let mut col_widths = vec![0; result.headers.len()];
    for (i, header) in result.headers.iter().enumerate() {
        col_widths[i] = header.len();
    }
    for row in &result.rows {
        for (i, field) in row.iter().enumerate() {
            col_widths[i] = max(col_widths[i], format_field(field).len());
        }
    }

    let mut lines = Vec::new();

    let sep: String = format!(
        "+-{}-+",
        col_widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("-+-")
    );

    lines.push(sep.clone());
    let header_line = format!(
        "| {} |",
        result
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:width$}", h, width = col_widths[i]))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    lines.push(header_line);
    lines.push(sep.clone());

    for row in &result.rows {
        let row_line = format!(
            "| {} |",
            row.iter()
                .enumerate()
                .map(|(i, f)| format!("{:width$}", format_field(f), width = col_widths[i]))
                .collect::<Vec<_>>()
                .join(" | ")
        );
        lines.push(row_line);
    }

    lines.push(sep);

    lines.push(format!("{} rows in set", result.rows.len()));

    lines.join("\n")
}

fn format_field(f: &DynamicField) -> String {
    match f {
        DynamicField::Integer(v) => v.to_string(),
        DynamicField::Text(s)
        | DynamicField::Phonennumber(s)
        | DynamicField::Email(s) => s.clone(),
    }
}
