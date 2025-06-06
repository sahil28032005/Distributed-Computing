use coordinator_lib::Coordinator;
use std::collections::HashMap;

#[tokio::test]
async fn test_sql_parsing() -> Result<(), Box<dyn std::error::Error>> {
    // Create a coordinator instance
    let mut coordinator = Coordinator::new();
    
    // Test a simple SELECT query
    let select_query = "SELECT id, name FROM users WHERE id = 1";
    let params = HashMap::new();
    let result = coordinator.execute_query(select_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "SELECT query should parse successfully");
    
    // Test an INSERT query
    let insert_query = "INSERT INTO users (id, name) VALUES (1, 'John Doe')";
    let result = coordinator.execute_query(insert_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "INSERT query should parse successfully");
    
    // Test an UPDATE query
    let update_query = "UPDATE users SET name = 'Jane Doe' WHERE id = 1";
    let result = coordinator.execute_query(update_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "UPDATE query should parse successfully");
    
    // Test a DELETE query
    let delete_query = "DELETE FROM users WHERE id = 1";
    let result = coordinator.execute_query(delete_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "DELETE query should parse successfully");
    
    // Test a CREATE TABLE query
    let create_table_query = "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255), email TEXT)";
    let result = coordinator.execute_query(create_table_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "CREATE TABLE query should parse successfully");
    
    // Test a query with parameters
    let mut params_with_values = HashMap::new();
    params_with_values.insert("user_id".to_string(), "1".to_string());
    let param_query = "SELECT * FROM users WHERE id = :user_id";
    let result = coordinator.execute_query(param_query.to_string(), params_with_values).await;
    assert!(result.is_ok(), "Parameter query should parse successfully");
    
    // Test an invalid query
    let invalid_query = "INVALID SQL STATEMENT";
    let result = coordinator.execute_query(invalid_query.to_string(), params.clone()).await;
    assert!(result.is_err(), "Invalid query should return an error");
    
    Ok(())
} 