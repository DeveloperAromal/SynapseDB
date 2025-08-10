use crate::storage::{page::Page, row::Row};
use serde::{Deserialize, Serialize};
use std::fs;

#[allow(dead_code)]
pub struct Table {
    pub table_id: u32,
    pub name: String,
    pub pages: Vec<Page>,
    pub max_rows_per_page: usize,
}
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub table_id: u32,
    pub name: String,
    pub num_of_pages: usize,
    pub max_rows_per_page: usize,
}

impl Table {
    #[allow(dead_code)]
    pub fn new(table_id: u32, name: String, max_rows_per_page: usize) -> Self {
        Self {
            table_id,
            name,
            pages: Vec::new(),
            max_rows_per_page,
        }
    }

    #[allow(dead_code)]
    pub fn insert_row(&mut self, row: Row) {
        if self.pages.len() == 0 {
            let mut page = Page::new(0);

            page.insert_row(row);
            self.pages.push(page);
        } else {
            let last_page = self.pages.last_mut().unwrap();

            if last_page.get_num_rows() < self.max_rows_per_page {
                last_page.insert_row(row);
            } else {
                let mut new_page = Page::new(self.pages.len() as u32);
                new_page.insert_row(row);
                self.pages.push(new_page);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_all_rows(&self) {
        for page in &self.pages {
            for row in &page.rows {
                println!("{:#?}", row)
            }
        }
    }

    pub fn get_num_rows(&self) -> usize {
        self.pages.iter().map(|page| page.get_num_rows()).sum()
    }

    #[allow(dead_code)]
    pub fn save_to_disk(&self) {
        let table_path =  format!("src/storage/tables/{}", self.name.clone());
        std::fs::create_dir_all(table_path).unwrap();
        for (i, page) in self.pages.iter().enumerate() {
            let data_byte: Vec<u8> = bincode::serialize(page).unwrap();

            let file_path = format!("src/storage/tables/{}/page_{}.bin", self.name.clone(), i);
            fs::write(file_path, data_byte).unwrap();
        }

        let table_metadata = Metadata {
            table_id: self.table_id,
            name: self.name.clone(),
            num_of_pages: self.pages.len(),
            max_rows_per_page: self.max_rows_per_page,
        };

        let meta_byte = bincode::serialize(&table_metadata).unwrap();
        let meta_path = format!("src/storage/tables/{}/metadata.bin", self.name.clone());
        fs::write(meta_path, meta_byte).unwrap();
    }
}
