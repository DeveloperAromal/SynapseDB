use super::ast::*;
use crate::query_processor::tokenizer;
use crate::storage::row::DynamicField;

const MAX_TABLE_NAME_LEN: usize = 64;

pub struct Parser {
    tokens: Vec<String>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) -> Option<String> {
        let tok = self.tokens.get(self.position).cloned();
        self.position += 1;
        tok
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        match self.next() {
            Some(ref t) if t.eq_ignore_ascii_case(expected) => Ok(()),
            Some(t) => Err(format!("Expected '{}', found '{}'", expected, t)),
            None => Err(format!("Expected '{}', found end of input", expected)),
        }
    }

    // Anything left over would otherwise be ignored without a word, so
    // `WHERE a = 1 AND b = 2` would quietly drop its second condition.
    fn expect_end(&mut self) -> Result<(), String> {
        if self.peek().map(|s| s.as_str()) == Some(";") {
            self.next();
        }
        match self.peek() {
            Some(t) => Err(format!("Unexpected '{}' after end of statement", t)),
            None => Ok(()),
        }
    }

    // The table name becomes a directory under synstore/tables, so it must not
    // be able to contain a path. It is stored lower case because SQL names are
    // case-insensitive and file systems do not agree on case.
    fn parse_table_name(&mut self) -> Result<String, String> {
        let name = self.next().ok_or("Expected table name")?;

        let mut chars = name.chars();
        let valid = name.len() <= MAX_TABLE_NAME_LEN
            && chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');

        if !valid {
            return Err(format!(
                "Invalid table name '{}': use up to {} letters, digits or underscores, not starting with a digit",
                name, MAX_TABLE_NAME_LEN
            ));
        }

        Ok(name.to_ascii_lowercase())
    }

    pub fn parse(sql: &str) -> Result<Query, String> {
        let tokens = tokenizer::tokensizer(sql)?;
        let mut parser = Parser::new(tokens);

        match parser.peek().map(|s| s.as_str()) {
            Some("SELECT") => parser.parse_select(),
            Some("INSERT") => parser.parse_insert(),
            Some("CREATE") => parser.parse_create(),
            Some("SWITCH") => parser.parse_switch(),
            Some(t) => Err(format!("Unknown query type '{}'", t)),
            None => Err("Empty query".to_string()),
        }
    }

    pub fn parse_select(&mut self) -> Result<Query, String> {
        self.expect("SELECT")?;

        let mut columns = Vec::new();
        if self.peek().map(|s| s.as_str()) == Some("*") {
            columns.push(SelectColumn::All);
            self.next();
        } else {
            loop {
                let col = self.next().ok_or("Expected column name")?;
                columns.push(SelectColumn::Column(col));
                if self.peek().map(|s| s.as_str()) != Some(",") {
                    break;
                }
                self.next();
            }
        }

        self.expect("FROM")?;
        let table_name = self.parse_table_name()?;

        let where_clause = if self.peek().map(|s| s.as_str()) == Some("WHERE") {
            self.next();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_end()?;

        Ok(Query::Select(Select {
            table_name,
            columns,
            where_clause,
        }))
    }

    pub fn parse_insert(&mut self) -> Result<Query, String> {
        self.expect("INSERT")?;
        self.expect("INTO")?;
        let table_name = self.parse_table_name()?;

        let mut columns = None;
        if self.peek().map(|s| s.as_str()) == Some("(") {
            self.next();
            let mut col_list = Vec::new();
            loop {
                let col = self.next().ok_or("Expected column name")?;
                if col == ")" {
                    break;
                }
                if col != "," {
                    col_list.push(col);
                }
            }
            columns = Some(col_list);
        }

        self.expect("VALUES")?;
        self.expect("(")?;

        let mut values = Vec::new();
        loop {
            let val = self.next().ok_or("Expected value")?;
            if val == ")" {
                break;
            }
            if val != "," {
                let value = match Self::parse_literal(&val)? {
                    Some(value) => value,
                    None => DynamicField::Text(val),
                };
                values.push(value);
            }
        }

        self.expect_end()?;

        Ok(Query::Insert(Insert {
            table_name,
            columns,
            value: vec![values],
        }))
    }

    pub fn parse_switch(&mut self) -> Result<Query, String> {
        self.expect("SWITCH")?;
        self.expect("SCHEMA")?;

        let schema_name = self.next().ok_or("Expected schema name")?;

        self.expect_end()?;

        Ok(Query::Switch(Switch {
            statement: CreateTable {
                table_name: schema_name,
                column: Vec::new(),
            },
        }))
    }

    pub fn parse_create(&mut self) -> Result<Query, String> {
        self.expect("CREATE")?;
        self.expect("TABLE")?;
        let table_name = self.parse_table_name()?;
        self.expect("(")?;

        let mut column_definitions = Vec::new();

        loop {
            let column_name = self.next().ok_or("Expected column name")?;
            if column_name == ")" {
                break;
            }
            if column_name == "," {
                continue;
            }

            let data_type_str = self.next().ok_or("Expected data type")?;
            let data_type = match data_type_str.to_uppercase().as_str() {
                "INTEGER" => DataType::Integer,
                "TEXT" => DataType::Text,
                "PHONENUMBER" => DataType::Phonenumber,
                "EMAIL" => DataType::Email,
                _ => return Err(format!("Unknown data type '{}'", data_type_str)),
            };

            column_definitions.push(ColumnDefinition {
                name: column_name,
                data_type,
                table_name: table_name.clone(),
            });

            if self.peek().map(|s| s.as_str()) == Some(")") {
                self.next();
                break;
            }
        }

        self.expect_end()?;

        Ok(Query::Create(Create {
            statement: CreateTable {
                table_name,
                column: column_definitions,
            },
        }))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let left_token = self.next().ok_or("Expected expression")?;
        let left = self.parse_operand(left_token)?;

        let operator: BinaryOperator = match self.next().ok_or("Expected operator")?.as_str() {
            "=" => BinaryOperator::Eq,
            "!=" => BinaryOperator::Neq,
            ">" => BinaryOperator::Gt,
            "<" => BinaryOperator::Lt,
            ">=" => BinaryOperator::Gte,
            "<=" => BinaryOperator::Lte,
            op => return Err(format!("Unknown operator '{}'", op)),
        };

        let right_token = self.next().ok_or("Expected RHS")?;
        let right = self.parse_operand(right_token)?;

        Ok(Expression::BinaryOp(
            Box::new(left),
            operator,
            Box::new(right),
        ))
    }



    fn parse_operand(&self, token: String) -> Result<Expression, String> {
        match Self::parse_literal(&token)? {
            Some(value) => Ok(Expression::Literal(value)),
            None => Ok(Expression::Identifier(token)),
        }
    }

    /// Parses an integer or a quoted string, and returns `None` for any other token.
    fn parse_literal(token: &str) -> Result<Option<DynamicField>, String> {
        // The tokenizer only produces a token starting with a quote for a
        // complete string literal, quotes included.
        if token.starts_with('\'') {
            return Ok(Some(DynamicField::Text(token[1..token.len() - 1].to_string())));
        }

        let digits = token.strip_prefix(['-', '+']).unwrap_or(token);
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Ok(None);
        }

        token
            .parse::<i32>()
            .map(|n| Some(DynamicField::Integer(n)))
            .map_err(|_| format!("Integer {} is out of range ({} to {})", token, i32::MIN, i32::MAX))
    }
}
