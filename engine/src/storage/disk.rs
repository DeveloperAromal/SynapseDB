use crate::storage::{page, row, table::Metadata};
use std::{fs, path::Path};

#[allow(dead_code)]
pub fn load_data(table_name: &str) -> Vec<row::Row> {
    let meta_path = format!("src/storage/tables/{}/metadata.bin", table_name);
    println!("{}", table_name);

    if !Path::new(&meta_path).exists() {
        println!("Metadata not found for {}", table_name);
        return Vec::new();
    }

    let read_meta_byte = fs::read(&meta_path).unwrap_or_else(|e| {
        println!("Failed to read metadata file: {}", e);
        Vec::new()
    });

    if read_meta_byte.is_empty() {
        println!("Metadata file is empty: {}", meta_path);
        return Vec::new();
    }
    let meta_data: Metadata = match bincode::deserialize(&read_meta_byte) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to deserialize metadata: {}", e);
            return Vec::new();
        }
    };

    if meta_data.name != table_name {
        println!("Table not found");
        return Vec::new();
    }

    let mut all_table_rows = Vec::new();

    for page_index in 0..meta_data.num_of_pages {
        let page_path = format!("src/storage/tables/{}/page_{}.bin", table_name, page_index);
        let page_bytes = match fs::read(&page_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Failed to read page file {}: {}", page_path, e);
                continue;
            }
        };

        if page_bytes.is_empty() {
            println!("Page file is empty: {}", page_path);
            continue;
        }

        let page::Page { rows, .. } = match bincode::deserialize(&page_bytes) {
            Ok(page) => page,
            Err(e) => {
                println!("Failed to deserialize page {}: {}", page_path, e);
                continue;
            }
        };

        all_table_rows.extend(rows);
    }

    all_table_rows
}
