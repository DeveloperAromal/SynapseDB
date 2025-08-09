use crate::storage::page::Page;

#[allow(dead_code)]
pub struct Table {
    table_id: u32,
    name: String,
    pages: Vec<Page>,
    max_rows_per_page: usize,
}

impl Table {
    #[allow(dead_code)]
    fn new(table_id: u32, name: String, max_rows_per_page: usize) -> Self {
        // let table = Table::new(table_id, name, max_rows_per_page);

        Self {
            table_id,
            name,
            pages: Vec::new(),
            max_rows_per_page,
        }
    }
}
