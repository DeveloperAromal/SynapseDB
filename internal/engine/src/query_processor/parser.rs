use crate::query_processor::tokenizer;
use crate::storage::row::DynamicField;
use super::ast::*;

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

    pub fn parse(sql: &str) -> Result<Query, String> {
        let tokens = tokenizer::tokensizer(sql);
        let mut parser = Parser::new(tokens);

        match parser.peek().map(|s| s.as_str()) {
            Some("SELECT") => parser.parse_select(),
            Some("INSERT") => parser.parse_insert(),
            Some("CREATE") => parser.parse_create(),
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
                let col = self.next().ok_or("Expected column name or '*'")?;
                columns.push(SelectColumn::Column(col));
                if self.peek().map(|s| s.as_str()) != Some(",") {
                    break;
                }
                self.next(); 
            }
        }

        self.expect("FROM")?;
        let table_name = self.next().ok_or("Expected table name")?;

        let where_clause = if self.peek().map(|s| s.as_str()) == Some("WHERE") {
            self.next();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Query::Select(Select {
            table_name,
            columns,
            where_clause,
        }))
    }

    pub fn parse_insert(&mut self) -> Result<Query, String> {
        self.expect("INSERT")?;
        self.expect("INTO")?;
        let table_name = self.next().ok_or("Expected table name after 'INTO'")?;
        
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
                if let Ok(num) = val.parse::<i64>() {
                    values.push(DynamicField::Integer(num.try_into().unwrap()));
                } else {
                    let cleaned_val = val.trim_matches('\'').to_string();
                    values.push(DynamicField::Text(cleaned_val));
                }
            }
        }
        
        Ok(Query::Insert(Insert {
            table_name,
            columns,
            value: vec![values], 
        }))
    }

    pub fn parse_create(&mut self) -> Result<Query, String> {
        self.expect("CREATE")?;
        self.expect("TABLE")?;
        let table_name = self.next().ok_or("Expected table name")?;
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
            let data_type_str = self.next().ok_or(format!("Expected data type for column '{}'", column_name))?;
            let data_type = match data_type_str.to_uppercase().as_str() {
                "INTEGER" => Ok(DataType::Integer),
                "TEXT" => Ok(DataType::Text),
                _ => Err(format!("Unknown data type '{}'", data_type_str)),
            }?;
            
            column_definitions.push(ColumnDefinition {
                name: column_name,
                data_type,
                table_name: table_name.clone(),
            });
            
            if self.peek().map(|s| s.as_str()) != Some(",") {
                if self.peek().map(|s| s.as_str()) == Some(")") {
                    self.next(); 
                    break;
                }
            }
        }

        Ok(Query::Create(Create {
            statement: CreateTable {
                table_name,
                column: column_definitions,
            }
        }))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let left = self.next().ok_or("Expected expression")?;
        let left_expr = self.parse_operand(left)?;
        
        let op_str = self.next().ok_or("Expected operator")?;
        let operator = match op_str.as_str() {
            "=" => BinaryOperator::Eq,
            "!=" => BinaryOperator::Neq,
            ">" => BinaryOperator::Gt,
            "<" => BinaryOperator::Lt,
            ">=" => BinaryOperator::Gte,
            "<=" => BinaryOperator::Lte,
            _ => return Err(format!("Unknown operator '{}'", op_str)),
        };

        let right = self.next().ok_or("Expected right-hand value")?;
        let right_expr = self.parse_operand(right)?;

        Ok(Expression::BinaryOp(
            Box::new(left_expr),
            operator,
            Box::new(right_expr),
        ))
    }
    
    fn parse_operand(&self, token: String) -> Result<Expression, String> {
        if let Ok(val) = token.parse::<i64>() {
            return Ok(Expression::Literal(DynamicField::Integer(val.try_into().unwrap())));
        }
        
        if token.starts_with('\'') && token.ends_with('\'') {
            let s = token.trim_matches('\'').to_string();
            return Ok(Expression::Literal(DynamicField::Text(s)));
        }
        
        Ok(Expression::Identifier(token))
    }
}
