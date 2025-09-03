mod query_processor;
mod storage;
use query_processor::ast::Query;
use query_processor::executor::execute_select;
use query_processor::parser::Parser;
use storage::disk::load_data;
use storage::row::{DynamicField, Field, Row};
use storage::table::Table;

fn main() {
    #[allow(dead_code)]
    let mut table = Table::new(1, "updates".to_string(), 2);

    let row1 = Row {
        fields: vec![
            Field {
                name: "id".to_string(),
                value: DynamicField::Integer(1),
            },
            Field {
                name: "name".to_string(),
                value: DynamicField::Text("Aromal".to_string()),
            },
        ],
    };

    let row2 = Row {
        fields: vec![
            Field {
                name: "id".to_string(),
                value: DynamicField::Integer(2),
            },
            Field {
                name: "name".to_string(),
                value: DynamicField::Text("Alice".to_string()),
            },
        ],
    };

    let row3 = Row {
        fields: vec![
            Field {
                name: "id".to_string(),
                value: DynamicField::Integer(3),
            },
            Field {
                name: "name".to_string(),
                value: DynamicField::Text("Bob".to_string()),
            },
        ],
    };

    table.insert_row(row1);
    table.insert_row(row2);
    table.insert_row(row3);

    println!("Total rows in table: {}", table.get_num_rows());

    println!("Number of pages in table: {}", table.get_num_pages());

    for (i, page) in table.get_pages().iter().enumerate() {
        println!("Page {} has {} rows", i, page.get_num_rows());
    }

    table.save_to_disk();

    let loaded_rows = load_data("updates");

    println!("\n--- Loaded Data ---\n");
    for row in loaded_rows.iter() {
        println!("{:?}", row);
    }
    println!("\n--- End of Data ---");

    let sql = "SELECT id, name FROM hack WHERE id > 1";

    match Parser::parse(sql) {
        Ok(Query::Select(select)) => {
            let result_rows = execute_select(&select);
            println!("--- Query Result ---");
            for row in result_rows.iter() {
                println!("{:?}", row);
            }
            println!("--- End of Query Result ---");
        }
        Ok(_) => println!("Only SELECT query is supported in this test."),
        Err(e) => println!("Parse error: {}", e),
    }
}
