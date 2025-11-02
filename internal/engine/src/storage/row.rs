use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum DynamicField {
    Integer(i32),
    Text(String),
    Phonennumber(String),
    Email(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: DynamicField,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub fields: Vec<Field>,
}
