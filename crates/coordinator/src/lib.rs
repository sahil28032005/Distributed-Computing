use std::sync::Arc;
use tokio::sync::Mutex;
use rpc::client::{DatabaseClient, NodeClient, RaftClient};
use common::error::Result;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::collections::HashMap;

/// Represents a node in the cluster
#[derive(Debug, Clone)]
pub struct Node {
    id: String,
    address: String,
    status: NodeStatus,
}

#[derive(Debug, Clone)]
pub enum NodeStatus {
    Active,
    Inactive,
    Failed,
}

/// The main coordinator service
pub struct Coordinator {
    nodes: Arc<Mutex<Vec<Node>>>,
    database_client: Option<DatabaseClient>,
    raft_client: Option<RaftClient>,
}

impl Coordinator {
    /// Create a new coordinator instance
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(Vec::new())),
            database_client: None,
            raft_client: None,
        }
    }

    /// Initialize the coordinator with necessary clients
    pub async fn initialize(&mut self, database_addr: &str, raft_addr: &str) -> Result<()> {
        // Connect to database service
        self.database_client = Some(DatabaseClient::connect(database_addr).await?);
        
        // Connect to raft service
        self.raft_client = Some(RaftClient::connect(raft_addr).await?);
        
        Ok(())
    }

    /// Add a new node to the cluster
    pub async fn add_node(&self, node: Node) -> Result<()> {
        let mut nodes = self.nodes.lock().await;
        nodes.push(node);
        Ok(())
    }

    /// Remove a node from the cluster
    pub async fn remove_node(&self, node_id: &str) -> Result<()> {
        let mut nodes = self.nodes.lock().await;
        nodes.retain(|node| node.id != node_id);
        Ok(())
    }

    /// Get the status of all nodes
    pub async fn get_nodes(&self) -> Vec<Node> {
        self.nodes.lock().await.clone()
    }

    /// Check the health of a specific node
    pub async fn check_node_health(&self, node_id: &str) -> Result<NodeStatus> {
        // Find the node
        let nodes = self.nodes.lock().await;
        let node = nodes.iter()
            .find(|n| n.id == node_id)
            .ok_or_else(|| common::error::DatabaseError::NotFound(format!("Node {} not found", node_id)))?;

        // Create a node client to check health
        let mut node_client = NodeClient::connect(&node.address).await?;
        
        // Get node status
        let status = node_client.get_status(()).await?;
        
        // Convert status to NodeStatus
        let node_status = match status.status.as_str() {
            "active" => NodeStatus::Active,
            "inactive" => NodeStatus::Inactive,
            _ => NodeStatus::Failed,
        };
        
        Ok(node_status)
    }

    /// Execute a database query
    pub async fn execute_query(&mut self, query: String, params: HashMap<String, String>) -> Result<()> {
        if let Some(db_client) = &mut self.database_client {
            let response = db_client.execute_query(query, params).await?;
            if !response.success {
                return Err(common::error::DatabaseError::QueryError(response.error));
            }
            Ok(())
        } else {
            Err(common::error::DatabaseError::NotConnected("Database client not initialized".into()))
        }
    }

    /// Get a value from the database
    pub async fn get_value(&mut self, key: String) -> Result<Vec<u8>> {
        if let Some(db_client) = &mut self.database_client {
            let response = db_client.get(key).await?;
            if !response.found {
                return Err(common::error::DatabaseError::NotFound("Key not found".into()));
            }
            Ok(response.value)
        } else {
            Err(common::error::DatabaseError::NotConnected("Database client not initialized".into()))
        }
    }

    /// Put a value in the database
    pub async fn put_value(&mut self, key: String, value: Vec<u8>) -> Result<()> {
        if let Some(db_client) = &mut self.database_client {
            let response = db_client.put(key, value).await?;
            if !response.success {
                return Err(common::error::DatabaseError::WriteError(response.error));
            }
            Ok(())
        } else {
            Err(common::error::DatabaseError::NotConnected("Database client not initialized".into()))
        }
    }

    /// Request a vote in the Raft consensus
    pub async fn request_vote(&mut self, term: u64, candidate_id: String) -> Result<bool> {
        if let Some(raft_client) = &mut self.raft_client {
            let response = raft_client.request_vote(term, candidate_id, 0, 0).await?;
            Ok(response.vote_granted)
        } else {
            Err(common::error::DatabaseError::NotConnected("Raft client not initialized".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_coordinator_creation() {
        let coordinator = Coordinator::new();
        assert!(coordinator.database_client.is_none());
        assert!(coordinator.raft_client.is_none());
    }
}

// Example usage
async fn example() -> Result<()> {
    // Create coordinator
    let mut coordinator = Coordinator::new();
    
    // Initialize with service addresses
    coordinator.initialize("localhost:50051", "localhost:50052").await?;
    
    // Add a node
    let node = Node {
        id: "node1".to_string(),
        address: "localhost:50053".to_string(),
        status: NodeStatus::Active,
    };
    coordinator.add_node(node).await?;
    
    // Execute a database query
    let mut params = HashMap::new();
    params.insert("table".to_string(), "users".to_string());
    coordinator.execute_query("SELECT * FROM users".to_string(), params).await?;
    
    // Get a value
    let value = coordinator.get_value("user:1".to_string()).await?;
    println!("Got value: {:?}", value);
    
    // Put a value
    coordinator.put_value("user:2".to_string(), b"John Doe".to_vec()).await?;
    
    // Request a vote
    let vote_granted = coordinator.request_vote(1, "node1".to_string()).await?;
    println!("Vote granted: {}", vote_granted);
    
    Ok(())
}

// HTTP server handlers
async fn get_nodes(coordinator: web::Data<Arc<Coordinator>>) -> impl Responder {
    let nodes = coordinator.get_nodes().await;
    HttpResponse::Ok().json(nodes)
}

async fn add_node(
    coordinator: web::Data<Arc<Coordinator>>,
    node: web::Json<Node>,
) -> impl Responder {
    match coordinator.add_node(node.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn start_http_server(coordinator: Arc<Coordinator>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(coordinator.clone()))
            .route("/nodes", web::get().to(get_nodes))
            .route("/nodes", web::post().to(add_node))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
