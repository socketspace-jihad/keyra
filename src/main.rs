use std::{cell::RefCell, collections::HashMap, rc::Rc};

use glommio::{channels::channel_mesh, net::TcpListener, LocalExecutorPoolBuilder, PoolPlacement};
use loggix::{error, with_fields};
use jemallocator::Jemalloc;
use keyra::handler;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
const ADDR: &str = "0.0.0.0:4000";

fn main() {
    let addr = ADDR;
    let num_cores = num_cpus::get();
    let channel_capacity = 1000;
    
    let builder = channel_mesh::FullMesh::full();

    let _ = with_fields!("addr".to_string() => &addr).info("keyra protocol is listening");
    
    LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(num_cores, None))
        .on_all_shards(move || {
            let storage = Rc::new(RefCell::new(HashMap::new()));
            let executor_id = glommio::executor().id();
            
            async move {
                let listener = TcpListener::bind(&addr).expect("Gagal bind");

                let _ = with_fields!(
                    "id".to_string() => executor_id,
                    "store_ptr".to_string() => format!("{:p}", storage.as_ptr()),
                ).info("core initialized with local store");

                loop {
                    let stream = match listener.accept().await {
                        Ok(s) => s,
                        Err(e) => {
                            error!(e.to_string());
                            continue;
                        }
                    };
                    
                    let store_for_client = storage.clone();
                    let num_shards_for_client = num_cores;

                    glommio::spawn_local(async move {
                        handler::handle_client(stream, store_for_client, num_shards_for_client).await;
                    }).detach();

                    glommio::yield_if_needed().await;
                }
            }
        })
        .expect("Gagal spawn pool")
        .join_all();
}
