use coordinator_lib::Coordinator;
use std::collections::HashMap;

#[tokio::test]
async fn test_sql_parser_integration() {
    // Create a coordinator
    let mut coordinator = Coordinator::new();
    let params: HashMap<String, String> = HashMap::new();
    
    // Test SELECT
    let select_query = "SELECT id, name FROM users";
    let result = coordinator.execute_query(select_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "SELECT query should parse successfully");
    
    // Test INSERT
    let insert_query = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
    let result = coordinator.execute_query(insert_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "INSERT query should parse successfully");
    
    // Test UPDATE
    let update_query = "UPDATE users SET name = 'Bob' WHERE id = 1";
    let result = coordinator.execute_query(update_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "UPDATE query should parse successfully");
    
    // Test DELETE
    let delete_query = "DELETE FROM users WHERE id = 1";
    let result = coordinator.execute_query(delete_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "DELETE query should parse successfully");
    
    // Test CREATE TABLE
    let create_table_query = "CREATE TABLE users (id INT PRIMARY KEY, name TEXT)";
    let result = coordinator.execute_query(create_table_query.to_string(), params.clone()).await;
    assert!(result.is_ok(), "CREATE TABLE query should parse successfully");
    
    // Test with parameters
    let param_query = "SELECT * FROM users WHERE id = :user_id";
    let mut params_with_values = HashMap::new();
    params_with_values.insert("user_id".to_string(), "1".to_string());
    let result = coordinator.execute_query(param_query.to_string(), params_with_values).await;
    assert!(result.is_ok(), "Parameterized query should parse successfully");
    
    // Test invalid SQL
    let invalid_query = "SELECT FROM WHERE";
    let result = coordinator.execute_query(invalid_query.to_string(), params.clone()).await;
    assert!(result.is_err(), "Invalid SQL should return an error");
} 