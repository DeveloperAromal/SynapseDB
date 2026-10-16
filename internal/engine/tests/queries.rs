//! End-to-end tests through `run_query`. They write real tables under
//! `synstore/tables` in the crate directory, each with its own name, and
//! remove them afterwards.

use engine::run_query;
use std::fs;
use std::path::Path;
use std::thread;

/// Deletes the table's files when dropped, so a failing test cleans up too.
struct TempTable(&'static str);

impl TempTable {
    fn new(name: &'static str) -> Self {
        let table = TempTable(name);
        table.remove();
        table
    }

    fn dir(&self) -> String {
        format!("synstore/tables/{}", self.0)
    }

    fn remove(&self) {
        let _ = fs::remove_dir_all(self.dir());
    }
}

impl Drop for TempTable {
    fn drop(&mut self) {
        self.remove();
    }
}

fn run_ok(sql: &str) -> String {
    let result = run_query(sql);
    assert!(!result.starts_with("Error") && !result.starts_with("Parse error"), "{} failed: {}", sql, result);
    result
}

#[test]
fn readme_example_round_trips() {
    let _table = TempTable::new("it_readme");

    assert_eq!(
        run_query("CREATE TABLE it_readme (id INTEGER, name TEXT, email EMAIL);"),
        "Table created: it_readme"
    );
    assert_eq!(
        run_query("INSERT INTO it_readme (id, name, email) VALUES (1, 'John Doe', 'john@example.com');"),
        "Insert succeeded into it_readme"
    );
    assert_eq!(
        run_query("SELECT * FROM it_readme;"),
        [
            "+----+----------+------------------+",
            "| id | name     | email            |",
            "+----+----------+------------------+",
            "| 1  | John Doe | john@example.com |",
            "+----+----------+------------------+",
            "1 rows in set",
        ]
        .join("\n")
    );
}

#[test]
fn where_matches_string_with_spaces() {
    let _table = TempTable::new("it_where");

    run_ok("CREATE TABLE it_where (id INTEGER, name TEXT)");
    run_ok("INSERT INTO it_where (id, name) VALUES (1, 'Mary Ann')");
    run_ok("INSERT INTO it_where (id, name) VALUES (2, 'Mary')");

    let result = run_ok("SELECT id FROM it_where WHERE name = 'Mary Ann'");
    assert!(result.contains("| 1  |"), "{}", result);
    assert!(result.ends_with("1 rows in set"), "{}", result);
}

#[test]
fn rows_are_kept_across_pages() {
    let table = TempTable::new("it_pages");

    run_ok("CREATE TABLE it_pages (n INTEGER)");
    for n in 1..=5 {
        run_ok(&format!("INSERT INTO it_pages (n) VALUES ({})", n));
    }

    // Two rows fit on a page, so five rows need three page files.
    assert!(Path::new(&format!("{}/page_2.bin", table.dir())).exists());

    let result = run_ok("SELECT * FROM it_pages WHERE n >= 2");
    assert!(result.ends_with("4 rows in set"), "{}", result);
}

#[test]
fn unknown_table_is_reported() {
    assert_eq!(run_query("SELECT * FROM it_missing"), "Error: Table 'it_missing' not found");
    assert_eq!(
        run_query("INSERT INTO it_missing (a) VALUES (1)"),
        "Error: Table 'it_missing' not found"
    );
}

#[test]
fn creating_an_existing_table_is_an_error() {
    let _table = TempTable::new("it_duplicate");

    run_ok("CREATE TABLE it_duplicate (a INTEGER)");
    assert_eq!(
        run_query("CREATE TABLE it_duplicate (a INTEGER)"),
        "Error: Table 'it_duplicate' already exists"
    );
}

#[test]
fn integer_out_of_range_does_not_panic() {
    let _table = TempTable::new("it_overflow");

    run_ok("CREATE TABLE it_overflow (a INTEGER)");
    let result = run_query("INSERT INTO it_overflow (a) VALUES (9999999999)");
    assert!(result.starts_with("Parse error: Integer 9999999999 is out of range"), "{}", result);
}

#[test]
fn rows_with_different_columns_are_aligned() {
    let _table = TempTable::new("it_ragged");

    run_ok("CREATE TABLE it_ragged (a INTEGER, b INTEGER)");
    run_ok("INSERT INTO it_ragged (a) VALUES (1)");
    run_ok("INSERT INTO it_ragged (a, b) VALUES (2, 3)");

    assert_eq!(
        run_query("SELECT * FROM it_ragged"),
        [
            "+---+------+",
            "| a | b    |",
            "+---+------+",
            "| 1 | NULL |",
            "| 2 | 3    |",
            "+---+------+",
            "2 rows in set",
        ]
        .join("\n")
    );

    let result = run_query("SELECT b, a FROM it_ragged");
    assert!(result.contains("| NULL | 1 |"), "{}", result);
}

#[test]
fn table_name_cannot_escape_the_data_directory() {
    let result = run_query("CREATE TABLE ../it_escape (a INTEGER)");
    assert!(result.starts_with("Parse error: Invalid table name"), "{}", result);
    assert!(!Path::new("synstore/it_escape").exists());
}

#[test]
fn table_names_are_case_insensitive() {
    let _table = TempTable::new("it_case");

    assert_eq!(run_query("CREATE TABLE It_Case (a INTEGER)"), "Table created: it_case");
    run_ok("INSERT INTO IT_CASE (a) VALUES (1)");
    assert!(run_ok("SELECT * FROM it_case").ends_with("1 rows in set"));
}

#[test]
fn concurrent_inserts_are_not_lost() {
    let _table = TempTable::new("it_concurrent");

    run_ok("CREATE TABLE it_concurrent (n INTEGER)");

    // Kept small because every INSERT rewrites every page of the table.
    let workers: Vec<_> = (0..4)
        .map(|worker| {
            thread::spawn(move || {
                for i in 0..3 {
                    run_ok(&format!("INSERT INTO it_concurrent (n) VALUES ({})", worker * 10 + i));
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }

    let result = run_ok("SELECT * FROM it_concurrent");
    assert!(result.ends_with("12 rows in set"), "{}", result.lines().last().unwrap_or(""));
}

#[test]
fn unreadable_page_fails_instead_of_dropping_rows() {
    let table = TempTable::new("it_missing_page");

    run_ok("CREATE TABLE it_missing_page (n INTEGER)");
    for n in 1..=3 {
        run_ok(&format!("INSERT INTO it_missing_page (n) VALUES ({})", n));
    }
    fs::remove_file(format!("{}/page_0.bin", table.dir())).unwrap();

    let result = run_query("INSERT INTO it_missing_page (n) VALUES (4)");
    assert!(result.starts_with("Error: Failed to read page file"), "{}", result);

    // The failed insert must not have rewritten the table without page 0.
    assert!(!Path::new(&format!("{}/page_0.bin", table.dir())).exists());
    assert!(run_query("SELECT * FROM it_missing_page").starts_with("Error: Failed to read page file"));
}
