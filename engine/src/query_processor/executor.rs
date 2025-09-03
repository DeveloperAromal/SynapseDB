use super::ast::*;
use crate::storage::disk::load_data;
use crate::storage::row::{DynamicField, Row};

pub fn execute_select(select: &Select) -> Vec<Vec<DynamicField>> {
    let rows = load_data(&select.table_name);

    let filtered_rows: Vec<_> = if let Some(expr) = &select.where_clause {
        rows.into_iter()
            .filter(|row| evaluate_expression(row, expr))
            .collect()
    } else {
        rows
    };

    filtered_rows
        .into_iter()
        .map(|row| select_columns(&row, &select.columns))
        .collect()
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
        Expression::Identifier(name) => row
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(name))
            .map(|f| f.value.clone()),
        Expression::BinaryOp(_, _, _) => None,
    }
}

fn select_columns(row: &Row, columns: &Vec<SelectColumn>) -> Vec<DynamicField> {
    match columns.as_slice() {
        [SelectColumn::All] => row.fields.iter().map(|f| f.value.clone()).collect(),
        _ => columns
            .iter()
            .filter_map(|c| match c {
                SelectColumn::All => None,
                SelectColumn::Column(name) => row
                    .fields
                    .iter()
                    .find(|f| f.name.eq_ignore_ascii_case(name))
                    .map(|f| f.value.clone()),
            })
            .collect(),
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
