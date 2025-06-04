use rpc::proto::database::database_service_client::DatabaseServiceClient;
use rpc::proto::database::{QueryRequest, GetRequest};

#[tokio::test]
async fn test_database_service() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let mut client = DatabaseServiceClient::connect("http://127.0.0.1:50051").await?;

    // Test execute_query
    let query_request = tonic::Request::new(QueryRequest {
        query: "SELECT * FROM test".to_string(),
        parameters: std::collections::HashMap::new(),
    });
    
    let response = client.execute_query(query_request).await?;
    println!("Query Response: {:?}", response);

    // Test get
    let get_request = tonic::Request::new(GetRequest {
        key: "test_key".to_string(),
    });
    
    let response = client.get(get_request).await?;
    println!("Get Response: {:?}", response);

    Ok(())
}
