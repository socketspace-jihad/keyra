use futures_lite::AsyncBufReadExt;
use glommio::{net::TcpListener, LocalExecutorPoolBuilder, PoolPlacement};
use futures_lite::{io::AsyncReadExt, io::AsyncWriteExt};

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

    loop {
        match stream.read_exact(&mut header).await {
            Ok(_) => {}
            Err(_) => break, // Koneksi putus
        }

        let op = header[0];
        let key_len = header[1] as usize;

        let mut key = [0u8;256];
        let mut val = [0u8;1024];

        if stream.read_exact(&mut key[..key_len]).await.is_err() { break; }
        
        match op {
            OP_SET => {
                let val_len = u16::from_be_bytes([header[2], header[3]]) as usize;
                if stream.read_exact(&mut val[..val_len]).await.is_err() { break; }
                let _ = stream.write(b"OK").await;
            }
            OP_GET => {
                let _ = stream.write(b"NF").await; // Not Found
            }
            _ => break,
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
