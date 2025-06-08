use coordinator_lib::{Coordinator, start_grpc_server};
use rpc::proto::database::database_service_client::DatabaseServiceClient;
use rpc::proto::database::QueryRequest;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

#[tokio::test]
async fn test_coordinator_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Create the coordinator
    let coordinator = Arc::new(Mutex::new(Coordinator::new()));
    
    // Start the gRPC server in a separate task
    let server_addr = "127.0.0.1:50052";
    let server_handle = tokio::spawn(async move {
        if let Err(e) = start_grpc_server(server_addr, coordinator).await {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Give the server some time to start
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Connect to the server
    let mut client = DatabaseServiceClient::connect(format!("http://{}", server_addr)).await?;
    
    // Create and send a request
    let mut parameters = HashMap::new();
    parameters.insert("id".to_string(), "1".to_string());
    
    let request = tonic::Request::new(QueryRequest {
        query: "SELECT id, name FROM users WHERE id = :id".to_string(),
        parameters,
    });
    
    // Send the request
    let response = client.execute_query(request).await?;
    let response = response.into_inner();
    
    println!("Response from server: {:?}", response);
    
    // Verify the response (this will be an empty result since our handlers are stubs)
    assert!(response.success, "Expected a successful response");
    
    // Cleanup
    drop(client);
    server_handle.abort();
    
    Ok(())
} 