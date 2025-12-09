use core::num;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use futures::{channel::mpsc::{self, Receiver, Sender}, StreamExt};
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
    let channel_capacity = 100000;
    let mut tx_chans = Vec::with_capacity(num_cores);
    let mut rx_chans = Vec::with_capacity(num_cores);
    for _ in 0..num_cores {
        let (tx,rx): (Sender<u8>,Receiver<u8>) = mpsc::channel(channel_capacity);
        tx_chans.push(tx);
        rx_chans.push(Mutex::new(Some(rx)));
    }
    let _ = with_fields!("addr".to_string() => &addr).info("keyra protocol is listening");
    let shared_receivers = Arc::new(rx_chans);
    
    LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(num_cores, None))
        .on_all_shards(move || {
            let storage = Rc::new(RefCell::new(HashMap::new()));
            let executor_id = glommio::executor().id();
            let rx = shared_receivers[executor_id-1]
                .lock()
                .unwrap()
                .take()
                .unwrap();
            glommio::spawn_local(async move {
                receival::handle_op(rx, executor_id).await;
            }).detach();
            
            async move {
                let listener = TcpListener::bind(&addr).expect("Gagal bind");

                let _ = with_fields!(
                    "id".to_string() => executor_id,
                    "store_ptr".to_string() => format!("{:p}", storage.as_ptr()),
                ).info("core initialized with local store");

                loop {
                    let sender_chan = tx_chans.clone();
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
                        handler::handle_client(stream, store_for_client, num_shards_for_client, sender_chan).await;
                    }).detach();

                    glommio::yield_if_needed().await;
                }
            }
        })
        .expect("Gagal spawn pool")
        .join_all();
}
