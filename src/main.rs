use futures_lite::io::BufReader;
use futures_lite::AsyncBufReadExt;
use glommio::{net::TcpListener, LocalExecutorPoolBuilder, PoolPlacement};
use futures_lite::{io::AsyncReadExt, io::AsyncWriteExt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const OP_SET: u8 = 1;
const OP_GET: u8 = 2;

async fn handle_client(
    mut stream: glommio::net::TcpStream,
) {
    let _ = stream.set_nodelay(true);
    let mut header = [0u8; 4];
    let mut stream = BufReader::new(stream);

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
                let _ = stream.write_all(b"OK").await;
            }
            OP_GET => {
                let _ = stream.write_all(b"NF").await; // Not Found
            }
            _ => break,
        }
        let buffer_is_empty = stream.fill_buf().await
            .map(|b| b.is_empty())
            .unwrap_or(true);

        if buffer_is_empty {
            if stream.flush().await.is_err() { break; }
        } else {
        }
    }
}

fn main() {
    LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(num_cpus::get(), None))
        .on_all_shards(|| async move {
            
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

                glommio::spawn_local(async move {
                    handle_client(stream).await;
                }).detach();
            }
        })
        .expect("Gagal spawn pool")
        .join_all(); // Tunggu semua core selesai (selamanya)
}
