use crate::storage::{page::Page, row::Row};

#[allow(dead_code)]
pub struct Table {
    table_id: u32,
    name: String,
    pages: Vec<Page>,
    max_rows_per_page: usize,
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
}
