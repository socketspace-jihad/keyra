// Copyright (c) 2013-2016 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::net::ToSocketAddrs;

use futures::AsyncReadExt;

mod hello_world_capnp {
    include!(concat!(env!("OUT_DIR"),"/hello_world_capnp.rs"));
}

use hello_world_capnp::hello_world;

mod data_capnp{
    include!(concat!(env!("OUT_DIR"),"/data_capnp.rs"));
}

use data_capnp::data;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 6 {
        println!("usage: {} client HOST:PORT MESSAGE", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    let op = args[3].to_string();
    let key = args[4].to_string();
    let value = args[5].to_string();

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;
            stream.set_nodelay(true)?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                futures::io::BufReader::new(reader),
                futures::io::BufWriter::new(writer),
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let data: data::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(rpc_system);

            match op.as_str() {
                "list" => {
                    let mut request = data.list_request();
                    let _ = request.send().promise.await?;
                    println!("sending list request..");
                },
                "set" => {
                    let mut request = data.receive_request();
                    request.get().init_key(key.len() as u32).push_str(key.as_str());
                    request.get().init_value().get_kind().init_string_val(value.len() as u32).push_str(value.as_str());
                    let _ = request.send().promise.await?;
                },
                "get" => {
                    let mut request = data.get_request();
                    request.get().init_key(key.len() as u32).push_str(key.as_str());
                    let result = request.send().promise.await?;
                    match result.get().unwrap().get_value().unwrap().get_kind().which().unwrap() {
                        data_capnp::types::kind::Which::StringVal(data) => {
                            println!("data {}",data.unwrap().to_string().unwrap());
                        },
                        _ => {
                            println!("not implemented yet");
                        }
                    }
                },
                _ => println!("method not found"),
            }
            
            println!(
                "completed!",
            );
            Ok(())
        })
        .await
}
