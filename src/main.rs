use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
mod core;
mod handler;


use futures::{future, AsyncReadExt};
use tokio::task;
use std::{error::Error, net::{SocketAddr, ToSocketAddrs}, str::FromStr};
use handler::capnp;



#[tokio::main(flavor="current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return Ok(());
    }

    println!("capnp server is listening on {}..",args[0]);

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    let addr2 = String::from("127.0.0.1:4001").to_socket_addrs().unwrap().next().unwrap();

    tokio::task::LocalSet::new()
        .run_until(async move {
            let capnp_listener = task::spawn_local(async move{
                capnp::capnp_server(&addr).await;
            });
            let grpc_listener = task::spawn_local(async move{
            });


            future::join(capnp_server_1,capnp_server_2).await;
        })
        .await;
    Ok(())
}
