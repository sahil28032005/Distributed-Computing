use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct ScanRequest {
    start_key: String,
    end_key: String,
    limit: usize,
}

/// Simple in-memory key-value store
struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }

    fn scan(&self, start_key: &str, end_key: &str, limit: usize) -> Vec<KeyValue> {
        self.data
            .iter()
            .filter(|(k, _)| k.as_str() >= start_key && k.as_str() <= end_key)
            .take(limit)
            .map(|(k, v)| KeyValue {
                key: k.clone(),
                value: v.clone(),
            })
            .collect()
    }
}

/// Database server implementation
pub struct DatabaseServer {
    store: Arc<Mutex<KeyValueStore>>,
}

impl DatabaseServer {
    /// Create a new database server
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(KeyValueStore::new())),
        }
    }

    /// Start the HTTP server
    pub async fn run(self, host: &str, port: u16) -> std::io::Result<()> {
        let store = self.store;
        
        HttpServer::new(move || {
            let store = store.clone();
            App::new()
                .app_data(web::Data::new(store))
                .route("/get/{key}", web::get().to(get_handler))
                .route("/put", web::post().to(put_handler))
                .route("/delete/{key}", web::delete().to(delete_handler))
                .route("/scan", web::post().to(scan_handler))
        })
        .bind((host, port))?
        .run()
        .await
    }
}

async fn get_handler(
    store: web::Data<Arc<Mutex<KeyValueStore>>>,
    key: web::Path<String>,
) -> impl Responder {
    let value = store.lock().await.get(&key);
    match value {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn put_handler(
    store: web::Data<Arc<Mutex<KeyValueStore>>>,
    kv: web::Json<KeyValue>,
) -> impl Responder {
    store.lock().await.put(kv.key.clone(), kv.value.clone());
    HttpResponse::Ok().finish()
}

async fn delete_handler(
    store: web::Data<Arc<Mutex<KeyValueStore>>>,
    key: web::Path<String>,
) -> impl Responder {
    store.lock().await.delete(&key);
    HttpResponse::Ok().finish()
}

async fn scan_handler(
    store: web::Data<Arc<Mutex<KeyValueStore>>>,
    req: web::Json<ScanRequest>,
) -> impl Responder {
    let items = store.lock().await.scan(&req.start_key, &req.end_key, req.limit);
    HttpResponse::Ok().json(items)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
