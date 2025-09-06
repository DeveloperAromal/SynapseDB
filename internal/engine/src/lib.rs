pub mod query_processor;
pub mod storage;

use query_processor::ast::Query;
use query_processor::executor::{execute_select, execute_insert, execute_create};
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
            let rows = execute_select(&select);
            format!("Rows: {:?}", rows)
        }
        #[allow(dead_code)]
        Ok(_) => "Unsupported query".to_string(),
        Err(e) => format!("Parse error: {}", e),
    }
}
