use std::{error::Error, net::SocketAddr};

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use tokio_util;

use crate::core;


mod data_capnp {
    include!(concat!(env!("OUT_DIR"),"/data_capnp.rs"));
}

use data_capnp::data;

pub async fn capnp_server(
    addr: &SocketAddr,
)->Result<(),Box<dyn Error>>{
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let data_client: data::Client = capnp_rpc::new_client(core::storage::Data);
    loop {
        let (stream, _) = listener.accept().await?;
        stream.set_nodelay(true)?;
        let (reader, writer) =
            tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let network = twoparty::VatNetwork::new(
            futures::io::BufReader::new(reader),
            futures::io::BufWriter::new(writer),
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        );
        let rpc_system =
            RpcSystem::new(Box::new(network), Some(data_client.clone().client));
        tokio::task::spawn_local(rpc_system);
    }
}
