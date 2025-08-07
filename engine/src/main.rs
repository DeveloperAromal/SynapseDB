mod storage;
use storage::row::{DynamicField, Field, Row, insert_row};
fn main() {
    
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

    insert_row(row);
}
