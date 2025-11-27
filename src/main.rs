

use futures::future;
use keyra::protocol::{binpro, grpc, http};
use tokio::{io::AsyncWriteExt, net::TcpListener, task::{self, JoinHandle}};

#[tokio::main(flavor="multi_thread")]
pub async fn main() -> std::io::Result<()> { 
    let listener = TcpListener::bind("0.0.0.0:4000").await?;

    println!("Listening on 0.0.0.0:4000");

    loop {
        match listener.accept().await {
            Ok((stream,addr)) =>{
                tokio::spawn(async move{
                    binpro::server::handle_read_vectored(stream).await;
                });
            },
            Err(_)=>{
                println!("Error");
            }
        }
    }

}
