use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

pub fn insert_row(row : Row) {

    let bytes: Vec<u8> = bincode::serialize(&row).unwrap();

    println!("{:#?}", bytes);


    let bytes_decode: Row = bincode::deserialize(&bytes).unwrap();

    println!("{:#?}", bytes_decode)

}
