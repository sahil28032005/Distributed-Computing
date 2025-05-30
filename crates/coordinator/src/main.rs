use coordinator_lib::DatabaseServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    let server = DatabaseServer::new();
    server.run("127.0.0.1", 8080).await
} 