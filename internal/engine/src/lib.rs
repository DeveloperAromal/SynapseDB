pub mod query_processor;
pub mod storage;

use query_processor::ast::Query;
use query_processor::executor::{execute_select, execute_insert, execute_create, SelectResult};
use storage::row::DynamicField;
use query_processor::parser::Parser;
use std::sync::Mutex;

// Statements read and rewrite whole tables under synstore/, and the server
// runs every connection on its own goroutine. Without this lock two INSERTs
// into one table can both start from the same old rows, and one is lost.
static ENGINE_LOCK: Mutex<()> = Mutex::new(());

pub fn run_query(query: &str) -> String {
    // A panic while the lock is held poisons it. The files on disk are the
    // only state, so later statements can still run safely.
    let _guard = ENGINE_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

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
            execute_select(&select)
                .map(format_select_result)
                .unwrap_or_else(|e| format!("Error: {}", e))
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

    // Widths are counted in characters because that is what `{:width$}` pads to.
    let mut col_widths: Vec<usize> = result.headers.iter().map(|h| h.chars().count()).collect();
    for row in &result.rows {
        for (i, field) in row.iter().enumerate() {
            col_widths[i] = max(col_widths[i], format_field(field).chars().count());
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

fn format_field(f: &Option<DynamicField>) -> String {
    match f {
        Some(DynamicField::Integer(v)) => v.to_string(),
        Some(DynamicField::Text(s))
        | Some(DynamicField::Phonennumber(s))
        | Some(DynamicField::Email(s)) => s.clone(),
        None => "NULL".to_string(),
    }
}
