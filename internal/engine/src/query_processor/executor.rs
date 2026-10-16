use super::ast::*;
use crate::storage::disk::{load_data, read_metadata};
use crate::storage::row::{DynamicField, Row, Field};
use crate::storage::table::{Metadata, Table};
use std::fs;

#[derive(Debug, Clone)]
pub struct SelectResult {
    pub headers: Vec<String>,
    /// One entry per header; `None` when the row has no such column.
    pub rows: Vec<Vec<Option<DynamicField>>>,
}

pub fn execute_select(select: &Select) -> Result<SelectResult, String> {
    let rows = load_data(&select.table_name)?;

    let filtered_rows: Vec<_> = if let Some(expr) = &select.where_clause {
        rows.into_iter()
            .filter(|row| evaluate_expression(row, expr))
            .collect()
    } else {
        rows
    };

    let headers = select_headers(&filtered_rows, &select.columns);

    let data_rows: Vec<Vec<Option<DynamicField>>> = filtered_rows
        .iter()
        .map(|row| headers.iter().map(|name| field_value(row, name)).collect())
        .collect();

    Ok(SelectResult { headers, rows: data_rows })
}

pub fn execute_insert(insert: &Insert) -> Result<(), String> {
    let metadata = read_metadata(&insert.table_name)?;

    let existing_rows = load_data(&insert.table_name)?;

    let new_rows = build_rows_from_insert(insert)?;

    let mut table = Table::new(metadata.table_id, insert.table_name.clone(), metadata.max_rows_per_page);

    for row in existing_rows.into_iter() {
        table.insert_row(row);
    }
    for row in new_rows.into_iter() {
        table.insert_row(row);
    }

    table.save_to_disk()
}

pub fn execute_create(create: &Create) -> Result<(), String> {
    let table_name = &create.statement.table_name;
    let dir_path = format!("synstore/tables/{}", table_name);

    if fs::metadata(&dir_path).is_ok() {
        return Err(format!("Table '{}' already exists", table_name));
    }

    std::fs::create_dir_all(&dir_path).map_err(|e| format!("Failed to create table dir: {}", e))?;

    let metadata = Metadata {
        table_id: 0,
        name: table_name.clone(),
        num_of_pages: 0,
        max_rows_per_page: 2,
    };

    let meta_bytes = bincode::serialize(&metadata).map_err(|e| format!("Serialize metadata failed: {}", e))?;
    let meta_path = format!("{}/metadata.bin", dir_path);
    fs::write(&meta_path, meta_bytes).map_err(|e| format!("Write metadata failed: {}", e))?;

    Ok(())
}

fn build_rows_from_insert(insert: &Insert) -> Result<Vec<Row>, String> {
    match &insert.columns {
        None => Err("INSERT requires explicit column list for now".to_string()),
        Some(col_names) => {
            let mut rows = Vec::new();
            for value_list in insert.value.iter() {
                if value_list.len() != col_names.len() {
                    return Err("Number of values does not match columns".to_string());
                }
                let mut fields = Vec::with_capacity(col_names.len());
                for (name, value) in col_names.iter().cloned().zip(value_list.iter().cloned()) {
                    fields.push(Field { name, value });
                }
                rows.push(Row { fields });
            }
            Ok(rows)
        }
    }
}

// Rows of one table can have different columns, because INSERT does not yet
// check them against the table definition, so `*` lists every column found.
fn select_headers(rows: &[Row], columns: &[SelectColumn]) -> Vec<String> {
    if !columns.contains(&SelectColumn::All) {
        return columns
            .iter()
            .filter_map(|c| match c {
                SelectColumn::Column(name) => Some(name.clone()),
                SelectColumn::All => None,
            })
            .collect();
    }

    let mut headers: Vec<String> = Vec::new();
    for field in rows.iter().flat_map(|row| &row.fields) {
        if !headers.iter().any(|h| h.eq_ignore_ascii_case(&field.name)) {
            headers.push(field.name.clone());
        }
    }
    headers
}

fn field_value(row: &Row, name: &str) -> Option<DynamicField> {
    row.fields
        .iter()
        .find(|f| f.name.eq_ignore_ascii_case(name))
        .map(|f| f.value.clone())
}

fn evaluate_expression(row: &Row, expr: &Expression) -> bool {
    match expr {
        Expression::BinaryOp(left, op, right) => {
            let lval = get_field_value(row, left);
            let rval = get_field_value(row, right);
            match op {
                BinaryOperator::Eq => lval.as_ref() == rval.as_ref(),
                BinaryOperator::Neq => lval.as_ref() != rval.as_ref(),
                BinaryOperator::Gt => compare_dynamic_fields(&lval, &rval).map(|o| o.is_gt()).unwrap_or(false),
                BinaryOperator::Lt => compare_dynamic_fields(&lval, &rval).map(|o| o.is_lt()).unwrap_or(false),
                BinaryOperator::Gte => compare_dynamic_fields(&lval, &rval).map(|o| o.is_ge()).unwrap_or(false),
                BinaryOperator::Lte => compare_dynamic_fields(&lval, &rval).map(|o| o.is_le()).unwrap_or(false),
            }
        }
        _ => true,
    }
}

fn get_field_value(row: &Row, expr: &Expression) -> Option<DynamicField> {
    match expr {
        Expression::Literal(v) => Some(v.clone()),
        Expression::Identifier(name) => field_value(row, name),
        Expression::BinaryOp(_, _, _) => None,
    }
}

#[derive(PartialEq, Eq)]
enum OrderingLike {
    Less,
    Equal,
    Greater,
}

impl OrderingLike {
    fn is_gt(&self) -> bool { matches!(self, OrderingLike::Greater) }
    fn is_lt(&self) -> bool { matches!(self, OrderingLike::Less) }
    fn is_ge(&self) -> bool { matches!(self, OrderingLike::Greater | OrderingLike::Equal) }
    fn is_le(&self) -> bool { matches!(self, OrderingLike::Less | OrderingLike::Equal) }
}

fn compare_dynamic_fields(left: &Option<DynamicField>, right: &Option<DynamicField>) -> Option<OrderingLike> {
    match (left.as_ref()?, right.as_ref()?) {
        (DynamicField::Integer(li), DynamicField::Integer(ri)) => Some(if li < ri {
            OrderingLike::Less
        } else if li > ri {
            OrderingLike::Greater
        } else {
            OrderingLike::Equal
        }),
        (DynamicField::Text(ls), DynamicField::Text(rs)) => Some(if ls < rs {
            OrderingLike::Less
        } else if ls > rs {
            OrderingLike::Greater
        } else {
            OrderingLike::Equal
        }),
        _ => None,
    }
}
