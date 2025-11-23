
use std::{self, net::SocketAddr};

use loggix::with_fields;
use tonic::transport::Server;

use super::handler;
use super::grpc_storage;


pub async fn run(addr: String)->Result<(),Box<dyn std::error::Error>>{
    with_fields!(
        "addr".to_string() => &addr
    ).info("grpc server is active");
    let addr: SocketAddr = addr.parse()?;
    let svc = handler::DataGrpc::default();
    Server::builder()
        .concurrency_limit_per_connection(1024)
        .max_concurrent_streams(1024)
        .initial_stream_window_size(512 * 1024)
        .initial_connection_window_size(1024 * 1024)
        .add_service(grpc_storage::key_value_server::KeyValueServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
