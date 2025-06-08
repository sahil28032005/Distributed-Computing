use peg;
use std::collections::HashMap;

/// SQL AST node types
#[derive(Debug, PartialEq, Clone)]
pub enum SqlStatement {
    Select {
        columns: Vec<String>,
        table: String,
        where_clause: Option<WhereClause>,
        limit: Option<usize>,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<SqlValue>,
    },
    Update {
        table: String,
        assignments: Vec<(String, SqlValue)>,
        where_clause: Option<WhereClause>,
    },
    Delete {
        table: String,
        where_clause: Option<WhereClause>,
    },
    CreateTable {
        name: String,
        columns: Vec<ColumnDef>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub primary_key: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Blob,
    Timestamp,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhereClause {
    pub condition: Condition,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    Comparison {
        left: String,
        op: ComparisonOp,
        right: SqlValue,
    },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Like,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SqlValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Parameter(String),
}

//this is main parser block it takes query and iterate query over statement functions as rules and mathch it with format specified in functions and parses it to rust speciffic data structure
peg::parser! {
    grammar sql_parser() for str {
        pub rule statement() -> SqlStatement
            = select_stmt()
            / insert_stmt()
            / update_stmt()
            / delete_stmt()
            / create_table_stmt()

        rule select_stmt() -> SqlStatement
            = whitespace()* "SELECT" whitespace()+ columns:column_list() whitespace()+ 
              "FROM" whitespace()+ table:identifier() 
              where_clause:where_clause()? 
              limit:limit_clause()?
              whitespace()* ";"? whitespace()* {
                SqlStatement::Select {
                    columns,
                    table,
                    where_clause,
                    limit,
                }
            }

        rule insert_stmt() -> SqlStatement
            = whitespace()* "INSERT" whitespace()+ "INTO" whitespace()+ table:identifier() 
              whitespace()* "(" whitespace()* columns:comma_list(<identifier()>) whitespace()* ")" whitespace()*
              "VALUES" whitespace()* "(" whitespace()* values:comma_list(<value()>) whitespace()* ")" whitespace()* ";"? whitespace()* {
                SqlStatement::Insert {
                    table,
                    columns,
                    values,
                }
            }

        rule update_stmt() -> SqlStatement
            = whitespace()* "UPDATE" whitespace()+ table:identifier() whitespace()+
              "SET" whitespace()+ assignments:comma_list(<assignment()>) 
              where_clause:where_clause()?
              whitespace()* ";"? whitespace()* {
                SqlStatement::Update {
                    table,
                    assignments,
                    where_clause,
                }
            }

        rule delete_stmt() -> SqlStatement
            = whitespace()* "DELETE" whitespace()+ "FROM" whitespace()+ table:identifier() 
              where_clause:where_clause()?
              whitespace()* ";"? whitespace()* {
                SqlStatement::Delete {
                    table,
                    where_clause,
                }
            }

        rule create_table_stmt() -> SqlStatement
            = whitespace()* "CREATE" whitespace()+ "TABLE" whitespace()+ name:identifier() 
              whitespace()* "(" whitespace()* columns:comma_list(<column_def()>) whitespace()* ")" 
              whitespace()* ";"? whitespace()* {
                SqlStatement::CreateTable {
                    name,
                    columns,
                }
            }

        rule column_def() -> ColumnDef
            = whitespace()* name:identifier() whitespace()+ data_type:data_type()
              nullable:nullable_def()? primary_key:primary_key_def()? whitespace()* {
                ColumnDef {
                    name,
                    data_type,
                    nullable: nullable.unwrap_or(true),
                    primary_key: primary_key.unwrap_or(false),
                }
            }

        rule nullable_def() -> bool
            = whitespace()+ "NOT" whitespace()+ "NULL" { false }
            / whitespace()+ "NULL" { true }

        rule primary_key_def() -> bool
            = whitespace()+ "PRIMARY" whitespace()+ "KEY" { true }

        rule data_type() -> DataType
            = "INT" { DataType::Int }
            / "INTEGER" { DataType::Int }
            / "FLOAT" { DataType::Float }
            / "DOUBLE" { DataType::Float }
            / "VARCHAR" whitespace()* "(" whitespace()* digits:$("0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"+) whitespace()* ")" { DataType::String }
            / "TEXT" { DataType::String }
            / "BOOLEAN" { DataType::Bool }
            / "BLOB" { DataType::Blob }
            / "TIMESTAMP" { DataType::Timestamp }

        rule assignment() -> (String, SqlValue)
            = whitespace()* col:identifier() whitespace()* "=" whitespace()* val:value() whitespace()* {
                (col, val)
            }

        rule where_clause() -> WhereClause
            = whitespace()+ "WHERE" whitespace()+ condition:condition() {
                WhereClause { condition }
            }

        rule condition() -> Condition
            = l:and_condition() whitespace()* "OR" whitespace()* r:condition() {
                Condition::Or(Box::new(l), Box::new(r))
            }
            / and_condition()

        rule and_condition() -> Condition
            = l:comparison() whitespace()* "AND" whitespace()* r:and_condition() {
                Condition::And(Box::new(l), Box::new(r))
            }
            / comparison()

        rule comparison() -> Condition
            = whitespace()* left:identifier() whitespace()* op:comparison_op() whitespace()* right:value() whitespace()* {
                Condition::Comparison { left, op, right }
            }

        rule comparison_op() -> ComparisonOp
            = "=" { ComparisonOp::Eq }
            / "!=" { ComparisonOp::NotEq }
            / "<>" { ComparisonOp::NotEq }
            / "<=" { ComparisonOp::LtEq }
            / "<" { ComparisonOp::Lt }
            / ">=" { ComparisonOp::GtEq }
            / ">" { ComparisonOp::Gt }
            / "LIKE" { ComparisonOp::Like }

        rule limit_clause() -> usize
            = whitespace()+ "LIMIT" whitespace()+ n:number() {
                n.parse().unwrap()
            }

        rule column_list() -> Vec<String>
            = "*" { vec!["*".to_string()] }
            / comma_list(<identifier()>)

        rule comma_list<T>(item: rule<T>) -> Vec<T>
            = first:item() rest:("," whitespace()* i:item() { i })* {
                let mut result = vec![first];
                result.extend(rest);
                result
            }

        rule value() -> SqlValue
            = s:string() { SqlValue::String(s) }
            / n:integer() { SqlValue::Integer(n.parse().unwrap()) }
            / f:float() { SqlValue::Float(f.parse().unwrap()) }
            / "TRUE" { SqlValue::Boolean(true) }
            / "FALSE" { SqlValue::Boolean(false) }
            / "NULL" { SqlValue::Null }
            / ":" p:identifier() { SqlValue::Parameter(p) }
            / "?" { SqlValue::Parameter("?".to_string()) }

        rule integer() -> &'input str
            = n:$(("-")? ("0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9")+) { n }

        rule float() -> &'input str
            = n:$(("-")? ("0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9")+ "." ("0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9")*) { n }

        rule string() -> String
            = "'" s:$((!("'") [_])*) "'" { s.to_string() }
            / "\"" s:$((!("\"") [_])*) "\"" { s.to_string() }

        rule identifier() -> String
            = quiet!{
                !keyword() id:$(("a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z") ("a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z" / "0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9" / "_")*) {
                    id.to_string()
                }
            }
            / "`" id:$((!"`" [_])*) "`" { id.to_string() }

        rule keyword() -> ()
            = whitespace()* ("SELECT" / "FROM" / "WHERE" / "INSERT" / "INTO" / "VALUES" / "UPDATE" 
                       / "SET" / "DELETE" / "CREATE" / "TABLE" / "AND" / "OR" / "NOT" / "NULL" 
                       / "PRIMARY" / "KEY" / "INT" / "INTEGER" / "FLOAT" / "VARCHAR" / "TEXT" 
                       / "BOOLEAN" / "BLOB" / "TIMESTAMP" / "LIMIT") whitespace()* {}

        rule number() -> &'input str
            = n:$(("0" / "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9")+) { n }

        rule whitespace()
            = quiet!{" " / "\t" / "\r" / "\n"}
    }
}

/// Parse a SQL statement string into an AST
pub fn parse_sql(input: &str) -> Result<SqlStatement, String> {
    match sql_parser::statement(input) {
        Ok(stmt) => Ok(stmt),
        Err(e) => Err(format!("SQL Parsing error: {}", e))
    }
}

/// Execute a SQL statement with parameters
pub fn execute_sql(
    _stmt: &SqlStatement, 
    _params: &HashMap<String, String>
) -> Result<(), String> {
    // Placeholder for actual execution logic
    // In a real implementation, this would connect to the storage layer
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_select() {
        let sql = "SELECT id, name FROM users;";
        let parsed = parse_sql(sql).unwrap();
        
        assert_eq!(
            parsed,
            SqlStatement::Select {
                columns: vec!["id".to_string(), "name".to_string()],
                table: "users".to_string(),
                where_clause: None,
                limit: None,
            }
        );
    }

    #[test]
    fn test_select_with_where() {
        let sql = "SELECT * FROM users WHERE id = 1;";
        let parsed = parse_sql(sql).unwrap();
        
        match parsed {
            SqlStatement::Select { columns, table, where_clause, .. } => {
                assert_eq!(columns, vec!["*".to_string()]);
                assert_eq!(table, "users");
                assert!(where_clause.is_some());
            },
            _ => panic!("Expected SELECT statement"),
        }
    }

    #[test]
    fn test_insert() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice');";
        let parsed = parse_sql(sql).unwrap();
        
        match parsed {
            SqlStatement::Insert { table, columns, values } => {
                assert_eq!(table, "users");
                assert_eq!(columns, vec!["id".to_string(), "name".to_string()]);
                assert_eq!(values.len(), 2);
            },
            _ => panic!("Expected INSERT statement"),
        }
    }
}
