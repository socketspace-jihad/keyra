use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use loggix::serde_json;

use crate::core::storage;


#[derive(serde::Serialize,serde::Deserialize,Debug)]
pub struct DataJson{
    pub key: String,
    pub value: storage::Types
}

#[derive(serde::Serialize, serde::Deserialize,Debug)]
pub struct SetData {
    pub key: String,
    pub value: storage::Types
}

#[derive(serde::Serialize, serde::Deserialize,Debug)]
pub struct  GetData{
    pub key: String,
}

pub async fn set(req: web::Json<SetData>)->impl Responder{
    storage::set(req.key.clone(),req.value.clone());
    HttpResponse::Ok().body("success")
}

pub async fn get(req: web::Json<GetData>)->impl Responder{
    match storage::get(&req.key) {
        Some(data) => {
            let value = serde_json::to_string(data.as_ref()).unwrap(); 
            HttpResponse::Ok().body(value)
        },
        None => {
            HttpResponse::NotFound().body("data not found")
        }
    }
}

