


use std::sync::LazyLock;

use super::grpc_storage;
use dashmap::DashMap;
use grpc_storage::{key_value_server::KeyValue, Empty, GetRequest, SetRequest, Types};
use tonic::{Request, Response, Status};


static STORAGE: LazyLock<DashMap<String,grpc_storage::Types>> = LazyLock::new(||{
    DashMap::new()
});

pub fn get(key: &String)->Option<grpc_storage::Types>{
    if let Some(data) = STORAGE.get(key) {
        return Some(data.clone());
    }
    return None;
}

pub fn set(key: String, value: grpc_storage::Types){
    STORAGE.insert(key, value);
}

#[derive(Default)]
pub struct DataGrpc;

#[tonic::async_trait]
impl KeyValue for DataGrpc {
    async fn get(&self, req: Request<GetRequest>)->Result<Response<Types>,Status> {
        if let Some(data) = get(&req.into_inner().key){
            return Ok(Response::new(data));
        }
        Err(Status::ok("not found".to_string()))
    }

    async fn set(&self, req: Request<SetRequest>)->Result<Response<Empty>,Status>{
        let inner = req.into_inner();
        if let Some(data) = inner.value {
            set(inner.key,data);
            return Ok(Response::new(grpc_storage::Empty{}))
        }
        Err(Status::ok("test".to_string()))
    }
}

