use futures::{channel::mpsc::{Receiver, Sender}, future::join_all};
use futures_lite::StreamExt;
use glommio::channels::{channel_mesh::Receivers, shared_channel::SharedReceiver};
use loggix::with_fields;


pub async fn handle_op(mut rx: Receivers<u8>,curr_idx: usize, cores: &usize){
    with_fields!(
        "core_id".to_string() => curr_idx,
        "peer_id".to_string() => rx.peer_id(),
    ).info("core mailbox is waiting for message");
    let mut streams = rx.streams().into_iter().collect::<Vec<_>>();

    let mut tasks = Vec::with_capacity(streams.len());

    for (producer_id, mut receiver) in streams.drain(..) {
        let task = glommio::spawn_local(async move {
            while let Some(msg) = receiver.next().await {
            }
        });
        tasks.push(task);
    }

    join_all(tasks).await;
}

pub async fn handle_response(rx: Receiver<u8>, tx: Sender<u8>){

}
