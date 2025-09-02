use crate::storage::row::DynamicField;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Query {
    Select(Select),
    Insert(Insert),
    Create(Create),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreateTable {
    pub table_name: String,
    pub column: Vec<ColumnDefinition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnDefinition {
    pub table_name: String,
    pub data_type: DataType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Integer,
    Text,
    Phonenumber,
    Email,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Insert {
    pub table_name: String,
    pub columns: Option<Vec<String>>,
    pub value: Vec<Vec<DynamicField>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Select {
    pub table_name: String,
    pub columns: Vec<SelectColumn>,
    pub where_clause: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SelectColumn {
    All,
    Column(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    Literal(DynamicField),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}


impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => write!(f, "INTEGER"),
            DataType::Text => write!(f, "TEXT"),
            DataType::Phonenumber => write!(f, "PHONENUMBER"),
            DataType::Email => write!(f, "EMAIL"),
        }
    }
}