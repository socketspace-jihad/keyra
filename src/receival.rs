use futures::channel::mpsc::{Receiver, Sender};
use futures_lite::StreamExt;
use glommio::channels::shared_channel::SharedReceiver;


pub async fn handle_op(mut rx: Receiver<u8>, core_id: usize){
//    while let Some(data) = rx.next().await {
//   }
}

pub async fn handle_response(rx: Receiver<u8>, tx: Sender<u8>){

}
