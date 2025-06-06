use crate::Coordinator;
use rpc::proto::database::database_service_server::{DatabaseService, DatabaseServiceServer};
use rpc::proto::database::{
    DeleteRequest, DeleteResponse, GetRequest, GetResponse, PutRequest, PutResponse,
    QueryRequest, QueryResponse, Row, ScanRequest, ScanResponse, Value,
};
use rpc::proto::node::node_service_server::{NodeService, NodeServiceServer};
use rpc::proto::node::{
    ReadRequest, ReadResponse, StatusRequest, StatusResponse, WriteRequest, WriteResponse,
};
use rpc::proto::raft::raft_service_server::{RaftService, RaftServiceServer};
use rpc::proto::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    RequestVoteRequest, RequestVoteResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::Request;
use tonic::Response;
use tonic::{transport::Server, Status};

// database service implementation
pub struct DatabaseServiceImpl {
    coordinator: Arc<Mutex<Coordinator>>,
}

impl DatabaseServiceImpl {
    pub fn new(coordinator: Arc<Mutex<Coordinator>>) -> Self {
        Self { coordinator }
    }

    // Helper to convert a hashmap row to a protobuf Row
    fn convert_to_proto_row(row: &std::collections::HashMap<String, String>) -> Row {
        let values = row.iter().map(|(_, value)| {
            // Simple conversion - in a real implementation, you'd determine the type
            Value {
                value: Some(rpc::proto::database::value::Value::StringValue(value.clone())),
            }
        }).collect();

        Row { values }
    }
}

#[tonic::async_trait]
impl DatabaseService for DatabaseServiceImpl {
    async fn execute_query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let req = request.into_inner();
        let mut coordinator = self.coordinator.lock().await;

        match coordinator.execute_query(req.query, req.parameters).await {
            Ok(result_rows) => {
                // Convert the result rows to protobuf format
                let rows = result_rows.iter()
                    .map(|row| Self::convert_to_proto_row(row))
                    .collect();

                Ok(Response::new(QueryResponse {
                    success: true,
                    error: "".to_string(),
                    rows,
                    affected_rows: result_rows.len() as u64,
                }))
            },
            Err(e) => Ok(Response::new(QueryResponse {
                success: false,
                error: e.to_string(),
                rows: vec![],
                affected_rows: 0,
            })),
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let mut coordinator = self.coordinator.lock().await;
        match coordinator.get(req.key).await {
            Ok(value) => Ok(Response::new(GetResponse {
                found: true,
                value,
                error: "".to_string(),
            })),
            Err(e) => Ok(Response::new(GetResponse {
                found: false,
                value: vec![],
                error: e.to_string(),
            })),
        }
    }

    async fn put(
        &self,
        request: Request<PutRequest>,
    ) -> Result<Response<PutResponse>, Status> {
        let req = request.into_inner();
        let mut coordinator = self.coordinator.lock().await;
        
        match coordinator.put(req.key, req.value).await {
            Ok(_) => Ok(Response::new(PutResponse {
                success: true,
                error: "".to_string(),
            })),
            Err(e) => Ok(Response::new(PutResponse {
                success: false,
                error: e.to_string(),
            })),
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let mut coordinator = self.coordinator.lock().await;
        
        match coordinator.delete(req.key).await {
            Ok(_) => Ok(Response::new(DeleteResponse {
                success: true,
                error: "".to_string(),
            })),
            Err(e) => Ok(Response::new(DeleteResponse {
                success: false,
                error: e.to_string(),
            })),
        }
    }

    async fn scan(
        &self,
        request: Request<ScanRequest>,
    ) -> Result<Response<ScanResponse>, Status> {
        let req = request.into_inner();
        let mut coordinator = self.coordinator.lock().await;
        
        match coordinator.scan(req.start_key, req.end_key, req.limit).await {
            Ok(items) => {
                let proto_items = items.into_iter()
                    .map(|(key, value)| rpc::proto::database::KeyValue { key, value })
                    .collect();
                
                Ok(Response::new(ScanResponse {
                    items: proto_items,
                    error: "".to_string(),
                }))
            },
            Err(e) => Ok(Response::new(ScanResponse {
                items: vec![],
                error: e.to_string(),
            })),
        }
    }
}

//Node service implementation
pub struct NodeServiceImpl {
    coordinator: Arc<Mutex<Coordinator>>,
}
impl NodeServiceImpl {
    pub fn new(coordinator: Arc<Mutex<Coordinator>>) -> Self {
        Self { coordinator }
    }
}
#[tonic::async_trait]
impl NodeService for NodeServiceImpl {
    async fn write(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement write")
    }

    async fn read(&self, request: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement read")
    }

    async fn scan(
        &self,
        request: Request<rpc::proto::node::ScanRequest>,
    ) -> Result<Response<rpc::proto::node::ScanResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement scan")
    }

    async fn get_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement get_status")
    }
}

// Raft service implementation
pub struct RaftServiceImpl {
    coordinator: Arc<Mutex<Coordinator>>,
}

impl RaftServiceImpl {
    //constructor like used while initializing it
    pub fn new(coordinator: Arc<Mutex<Coordinator>>) -> Self {
        Self { coordinator }
    }
}

#[tonic::async_trait]
impl RaftService for RaftServiceImpl {
    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement append_entries")
    }

    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement request_vote")
    }

    async fn install_snapshot(
        &self,
        request: Request<InstallSnapshotRequest>,
    ) -> Result<Response<InstallSnapshotResponse>, Status> {
        let req=request.into_inner();
        todo!("Implement install_snapshot")
    }
}

pub async fn start_grpc_server(
    addr: &str,
    coordinator: Arc<Mutex<Coordinator>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_service = DatabaseServiceImpl::new(coordinator.clone());
    let node_service = NodeServiceImpl::new(coordinator.clone());
    let raft_service = RaftServiceImpl::new(coordinator.clone());

    println!("Starting gRPC server on {}...", addr);
    Server::builder()
        .add_service(DatabaseServiceServer::new(db_service))
        .add_service(NodeServiceServer::new(node_service))
        .add_service(RaftServiceServer::new(raft_service))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}