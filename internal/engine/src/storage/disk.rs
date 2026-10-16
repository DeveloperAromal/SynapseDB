use crate::storage::{page, row, table::Metadata};
use std::{fs, io};

// Errors are returned rather than skipped: INSERT rewrites the whole table from
// what this returns, so a page silently left out would be deleted from disk.
#[allow(dead_code)]
pub fn load_data(table_name: &str) -> Result<Vec<row::Row>, String> {
    let meta_data = read_metadata(table_name)?;

    let mut all_table_rows = Vec::new();

    for page_index in 0..meta_data.num_of_pages {
        let page_path = format!("synstore/tables/{}/page_{}.bin", table_name, page_index);
        let page_bytes = fs::read(&page_path)
            .map_err(|e| format!("Failed to read page file {}: {}", page_path, e))?;

        let page::Page { rows, .. } = bincode::deserialize(&page_bytes)
            .map_err(|e| format!("Failed to deserialize page {}: {}", page_path, e))?;

        all_table_rows.extend(rows);
    }

    Ok(all_table_rows)
}

pub fn read_metadata(table_name: &str) -> Result<Metadata, String> {
    let meta_path = format!("synstore/tables/{}/metadata.bin", table_name);

    let read_meta_byte = match fs::read(&meta_path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(format!("Table '{}' not found", table_name));
        }
        Err(e) => return Err(format!("Failed to read metadata file {}: {}", meta_path, e)),
    };

    let meta_data: Metadata = bincode::deserialize(&read_meta_byte)
        .map_err(|e| format!("Failed to deserialize metadata {}: {}", meta_path, e))?;

    // Table names are case-insensitive, and tables created before names were
    // lower-cased may have stored their original spelling.
    if !meta_data.name.eq_ignore_ascii_case(table_name) {
        return Err(format!("Table '{}' not found", table_name));
    }

    Ok(meta_data)
}
