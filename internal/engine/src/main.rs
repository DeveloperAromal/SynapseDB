mod query_processor;
mod storage;
use query_processor::ast::Query;
use query_processor::executor::{execute_select, execute_insert, execute_create};
use query_processor::parser::Parser;

fn main() {
    let create_sql = "CREATE TABLE test101 (id INTEGER, name TEXT)";
    match Parser::parse(create_sql) {
        Ok(Query::Create(create)) => {
            match execute_create(&create) {
                Ok(_) => println!("Table created: {}", create.statement.table_name),
                Err(e) => println!("Create error: {}", e),
            }
        }
        _ => {}
    }

    let insert_sql = "INSERT INTO test101 (id, name) VALUES (4, 'Carol')";
    match Parser::parse(insert_sql) {
        Ok(Query::Insert(insert)) => {
            match execute_insert(&insert) {
                Ok(_) => println!("Insert succeeded into {}", insert.table_name),
                Err(e) => println!("Insert error: {}", e),
            }
        }
        _ => {}
    }

    let sql = "SELECT id, name FROM test101 WHERE id > 1";

    match Parser::parse(sql) {
        Ok(Query::Select(select)) => {
            let result_rows = execute_select(&select);
            println!("--- Query Result ---");
            for row in result_rows.iter() {
                println!("{:?}", row);
            }
            println!("--- End of Query Result ---");
        }
        Ok(_) => println!("Only SELECT query is supported in this test101."),
        Err(e) => println!("Parse error: {}", e),
    }
}
