use crate::storage::{page, row};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TableMetadata {
    pub name: String,
    pub num_of_pages: u32,
}

#[allow(dead_code)]
pub fn load_data(table_name: &str) -> Vec<row::Row> {
    let meta_path = format!("src/storage/tables/{}/metadata.bin", table_name);
    let read_meta_byte = fs::read(meta_path).unwrap();
    let meta_data: TableMetadata = bincode::deserialize(&read_meta_byte).unwrap();

    if meta_data.name != table_name {
        panic!("Table not found");
    }

    let mut all_table_rows = Vec::new();

    for page_index in 0..meta_data.num_of_pages {
        let page_path = format!("src/storage/tables/{}/page_{}.bin", table_name, page_index);
        let page_bytes = fs::read(page_path).unwrap();

        let page::Page { rows, .. } = bincode::deserialize(&page_bytes).unwrap();
        all_table_rows.extend(rows);
    }

    all_table_rows
}
