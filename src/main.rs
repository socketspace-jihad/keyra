use glommio::{net::TcpListener, LocalExecutorPoolBuilder, PoolPlacement};
use futures_lite::{io::AsyncReadExt, io::AsyncWriteExt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const OP_SET: u8 = 1;
const OP_GET: u8 = 2;

async fn handle_client(
    mut stream: glommio::net::TcpStream,
    store: Rc<RefCell<HashMap<Vec<u8>, Vec<u8>>>>
) {
    let mut header = [0u8; 4];

    loop {
        match stream.read_exact(&mut header).await {
            Ok(_) => {}
            Err(_) => break, // Koneksi putus
        }

        let op = header[0];
        let key_len = header[1] as usize;
        let val_len = u16::from_be_bytes([header[2], header[3]]) as usize;

        let mut key = vec![0u8; key_len];
        let mut val = vec![0u8; val_len];

        if stream.read_exact(&mut key).await.is_err() { break; }
        
        match op {
            OP_SET => {
                if stream.read_exact(&mut val).await.is_err() { break; }
                
                store.borrow_mut().insert(key, val);
                
                let _ = stream.write_all(b"OK").await;
            }
            OP_GET => {
                let db = store.borrow();
                if let Some(value) = db.get(&key) {
                    let _ = stream.write_all(value).await;
                } else {
                    let _ = stream.write_all(b"NF").await; // Not Found
                }
            }
            _ => break,
        }
    }
}

fn main() {
    LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(num_cpus::get(), None))
        .on_all_shards(|| async move {
            let local_store = Rc::new(RefCell::new(HashMap::new()));
            
            let listener = TcpListener::bind("0.0.0.0:4000").expect("Gagal bind");
            println!("Core aktif, listening on port 4000...");

            loop {
                let stream = match listener.accept().await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Accept error: {}", e);
                        continue;
                    }
                };

                let store_clone = local_store.clone();
                glommio::spawn_local(async move {
                    handle_client(stream, store_clone).await;
                }).detach();
            }
        })
        .expect("Gagal spawn pool")
        .join_all(); // Tunggu semua core selesai (selamanya)
}
