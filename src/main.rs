use std::{cell::RefCell, collections::HashMap, rc::Rc};

use glommio::{channels::channel_mesh, net::TcpListener, LocalExecutorPoolBuilder, PoolPlacement};
use loggix::{error, with_fields};
use jemallocator::Jemalloc;
use keyra::{handler, receival};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
const ADDR: &str = "0.0.0.0:4000";

fn main() {
    let addr = ADDR;
    let num_cores = num_cpus::get();
    let mesh = channel_mesh::MeshBuilder::full(num_cores, 100_000);
    
    LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(num_cores, None))
        .on_all_shards(move ||{
            let storage = Rc::new(RefCell::new(HashMap::new()));
            let executor_id = glommio::executor().id()-1;
            
            async move {
                let (tx,rx) = mesh.join().await.unwrap();
                glommio::spawn_local(async move{
                    receival::handle_op(rx,executor_id, &num_cores).await;
                }).detach();
                let listener = TcpListener::bind(&addr).expect("Gagal bind");

                let _ = with_fields!(
                    "id".to_string() => executor_id,
                    "store_ptr".to_string() => format!("{:p}", storage.as_ptr()),
                ).info("core initialized with local store");
                let num_shards_for_client = num_cores;
                let mut tx = Rc::new(tx);
                loop {
                    let mut tx = tx.clone();
                    let stream = match listener.accept().await {
                        Ok(s) => s,
                        Err(e) => {
                            error!(e.to_string());
                            continue;
                        }
                    };
                    let storage = storage.clone();
                    glommio::spawn_local(async move{
                        handler::handle_client(stream, storage, &num_shards_for_client,tx).await;
                    }).detach();

                    glommio::yield_if_needed().await;
                }
            }
        })
        .expect("Gagal spawn pool")
        .join_all();
}
