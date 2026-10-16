use engine::query_processor::ast::*;
use engine::query_processor::parser::Parser;
use engine::query_processor::tokenizer::tokensizer;
use engine::storage::row::DynamicField;

fn tokens(sql: &str) -> Vec<String> {
    tokensizer(sql).expect("tokenizer failed")
}

fn parse_insert_values(sql: &str) -> Vec<DynamicField> {
    match Parser::parse(sql) {
        Ok(Query::Insert(insert)) => insert.value.into_iter().next().unwrap(),
        other => panic!("expected an INSERT, got {:?}", other),
    }
}

#[test]
fn string_literal_with_spaces_and_punctuation_is_one_token() {
    assert_eq!(
        tokens("VALUES ('John Doe', 'a, (b); c')"),
        ["VALUES", "(", "'John Doe'", ",", "'a, (b); c'", ")"]
    );
}

#[test]
fn doubled_quote_inside_string_literal_is_an_escaped_quote() {
    assert_eq!(tokens("'O''Brien'"), ["'O'Brien'"]);
    assert_eq!(tokens("''"), ["''"]);
}

#[test]
fn unterminated_string_literal_is_an_error() {
    assert!(tokensizer("SELECT * FROM t WHERE name = 'abc").is_err());
    assert!(tokensizer("'abc''").is_err());
}

#[test]
fn any_whitespace_separates_tokens() {
    assert_eq!(tokens("SELECT\r\n*\tFROM\r\nusers"), ["SELECT", "*", "FROM", "users"]);
}

#[test]
fn keywords_inside_string_literals_are_left_alone() {
    assert_eq!(tokens("select 'select'"), ["SELECT", "'select'"]);
}

#[test]
fn insert_parses_strings_and_integers() {
    assert_eq!(
        parse_insert_values("INSERT INTO users (id, name, note, bare) VALUES (-7, 'John Doe', 'it''s', bare);"),
        [
            DynamicField::Integer(-7),
            DynamicField::Text("John Doe".to_string()),
            DynamicField::Text("it's".to_string()),
            DynamicField::Text("bare".to_string()),
        ]
    );
}

#[test]
fn integer_out_of_range_is_an_error_not_a_panic() {
    let err = Parser::parse("INSERT INTO t (a) VALUES (9999999999)").unwrap_err();
    assert!(err.contains("out of range"), "unexpected error: {}", err);

    let err = Parser::parse("SELECT * FROM t WHERE a = -2147483649").unwrap_err();
    assert!(err.contains("out of range"), "unexpected error: {}", err);
}

#[test]
fn integer_limits_are_accepted() {
    assert_eq!(
        parse_insert_values("INSERT INTO t (a, b) VALUES (2147483647, -2147483648)"),
        [DynamicField::Integer(i32::MAX), DynamicField::Integer(i32::MIN)]
    );
}

#[test]
fn where_compares_against_quoted_string() {
    match Parser::parse("SELECT name FROM users WHERE name = 'Mary Ann'") {
        Ok(Query::Select(select)) => assert_eq!(
            select.where_clause,
            Some(Expression::BinaryOp(
                Box::new(Expression::Identifier("name".to_string())),
                BinaryOperator::Eq,
                Box::new(Expression::Literal(DynamicField::Text("Mary Ann".to_string()))),
            ))
        ),
        other => panic!("expected a SELECT, got {:?}", other),
    }
}

#[test]
fn trailing_semicolon_is_accepted() {
    assert!(Parser::parse("SELECT * FROM users;").is_ok());
    assert!(Parser::parse("CREATE TABLE users (id INTEGER);").is_ok());
}

#[test]
fn tokens_after_the_statement_are_rejected() {
    for sql in [
        "SELECT * FROM users WHERE age > 30 AND age < 40",
        "SELECT * FROM users ORDER BY age",
        "SELECT * FROM users; SELECT * FROM users",
        "INSERT INTO users (id) VALUES (1) extra",
        "CREATE TABLE users (id INTEGER) extra",
    ] {
        let err = Parser::parse(sql).unwrap_err();
        assert!(err.contains("after end of statement"), "{}: unexpected error {}", sql, err);
    }
}

#[test]
fn create_accepts_every_data_type() {
    match Parser::parse("CREATE TABLE contacts (id INTEGER, name TEXT, phone PHONENUMBER, email EMAIL)") {
        Ok(Query::Create(create)) => {
            let types: Vec<DataType> = create.statement.column.into_iter().map(|c| c.data_type).collect();
            assert_eq!(types, [DataType::Integer, DataType::Text, DataType::Phonenumber, DataType::Email]);
        }
        other => panic!("expected a CREATE, got {:?}", other),
    }
}

#[test]
fn table_names_that_are_not_plain_identifiers_are_rejected() {
    for sql in [
        "CREATE TABLE ../escape (a INTEGER)",
        "CREATE TABLE a/b (a INTEGER)",
        "CREATE TABLE a\\b (a INTEGER)",
        "CREATE TABLE 1abc (a INTEGER)",
        "SELECT * FROM ..",
        "INSERT INTO C:x (a) VALUES (1)",
    ] {
        let err = Parser::parse(sql).unwrap_err();
        assert!(err.starts_with("Invalid table name"), "{}: unexpected error {}", sql, err);
    }

    let long_name = "a".repeat(65);
    assert!(Parser::parse(&format!("SELECT * FROM {}", long_name)).is_err());
}

#[test]
fn table_names_are_lower_cased() {
    match Parser::parse("SELECT * FROM Users") {
        Ok(Query::Select(select)) => assert_eq!(select.table_name, "users"),
        other => panic!("expected a SELECT, got {:?}", other),
    }
}
