use coordinator_lib::Coordinator;
use tokio::signal;
use tokio::sync::Mutex;
use std::sync::Arc;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting coordinator...");
    
    // Create a new coordinator instance
    let coordinator = Arc::new(Mutex::new(Coordinator::new()));
    
    // Initialize the coordinator (if needed)
    {
        let coord = coordinator.lock().await;
        // You might want to load configuration here
        // coord.initialize("localhost:50051", "localhost:50052").await?;
    }
    
    // Start the gRPC server
    let server_addr = "127.0.0.1:50051";
    let server_handle = tokio::spawn(server::start_grpc_server(
        server_addr,
        coordinator.clone(),
    ));
    
    println!("Coordinator running on {}. Press Ctrl+C to exit.", server_addr);
    
    // Wait for Ctrl+C
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Shutting down coordinator...");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
    
    // Wait for the server to finish
    let _ = server_handle.await;
    
    Ok(())
}