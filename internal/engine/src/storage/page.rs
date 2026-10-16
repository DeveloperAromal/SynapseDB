use crate::storage::row::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Page {
    pub page_id: u32,
    pub row_id: usize,
    pub rows: Vec<Row>,
}

impl Page {
    #[allow(dead_code)]
    pub fn new(page_id: u32) -> Self {
        Self {
            page_id,
            row_id: 0,
            rows: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_table_by_id(&self) -> u32 {
        self.page_id
    }



    #[allow(dead_code)]
    pub fn get_num_rows(&self) -> usize{
        self.rows.len()
    }


    #[allow(dead_code)]
    pub fn insert_row(&mut self, row: Row) {
        self.rows.push(row);
        self.row_id += 1;
    }
}
