mod storage;
use storage::row::{DynamicField, Field, Row};

use crate::storage::page::Page;

fn main() {
    #[allow(dead_code)]
    let mut page = Page::new(1);


    println!("This is befor insertion: {}", page.rows.len());

    let row = Row {
        fields: vec![
            Field {
                name: "id".to_string(),
                value: DynamicField::Integer(1),
            },
            Field {
                name: "name".to_string(),
                value: DynamicField::Text("Aromal".to_string()),
            },
            Field {
                name: "Phonenumber".to_string(),
                value: DynamicField::Phonennumber(956999997.to_string()),
            },
            Field {
                name: "email".to_string(),
                value: DynamicField::Email("developeraromal@gmail.com".to_string()),
            },
        ],
    };


    page.insert_row(row);
    println!("Insertion sucessfull");
    println!("This is after insertion: {}", page.rows.len());

}
