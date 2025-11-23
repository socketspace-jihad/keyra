

use std::future::Future;

use futures::future;
use keyra::protocol::{binpro, grpc, http};
use tokio::task::{self, JoinHandle};


#[tokio::main(flavor="multi_thread",worker_threads = 16)]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = keyra::config::load_config()?;
    binpro::server::run();
    
    Ok(())
}
