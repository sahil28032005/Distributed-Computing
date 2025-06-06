use rpc::proto::database::database_service_client::DatabaseServiceClient;
use rpc::proto::database::{QueryRequest, QueryResponse};
use std::collections::HashMap;

#[tokio::test]
async fn test_execute_query() -> Result<(), Box<dyn std::error::Error>> {
    // Start a client connection to the server
    let mut client = DatabaseServiceClient::connect("http://127.0.0.1:50051").await?;
    
    // Create a SQL query request
    let mut parameters = HashMap::new();
    parameters.insert("param1".to_string(), "value1".to_string());
    
    let request = tonic::Request::new(QueryRequest {
        query: "SELECT id, name FROM users WHERE id = :param1".to_string(),
        parameters,
    });
    
    // Send the request and get the response
    let response = client.execute_query(request).await?;
    let response = response.into_inner();
    
    println!("Response: {:?}", response);
    
    Ok(())
} 