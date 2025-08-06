
#[allow(dead_code)]
#[derive(Debug)]
enum DynamicField {
    Integer(i32),
    Text(String),
    Phonennumber(String),
    Email(String),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Field {
    name: String,
    value: DynamicField,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Row {
    fields: Vec<Field>,
}

pub fn insert_row() {
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

    println!("{:#?}", row);

    println!("Rows inserted")
}
