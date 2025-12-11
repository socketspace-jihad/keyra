use std::rc::Rc;

use futures::channel::mpsc::Sender;
use futures::SinkExt;
use futures_lite::io::{AsyncReadExt,AsyncWriteExt,BufReader, BufWriter};
use glommio::channels::channel_mesh::Senders;
use crate::storage;
use crate::types::{SharedStore};
use crate::sharding::get_shard_id;

const OP_SET: u8 = 1;
const OP_GET: u8 = 2;
const MAX_PAYLOAD_BUFFER: usize = 1024;


#[inline(always)]
pub async fn handle_client(stream: glommio::net::TcpStream, db: SharedStore, num_shards: &usize, sender: Rc<Senders<u8>>) {
    let core_id = glommio::executor().id()-1;
    let _ = stream.set_nodelay(true);
    let (raw_reader, raw_writer) = futures_lite::io::split(stream);
    let mut reader = BufReader::with_capacity(65536, raw_reader);
    let mut writer = BufWriter::with_capacity(65536, raw_writer);
    let mut buffer = [0u8; MAX_PAYLOAD_BUFFER];
    let mut header = [0u8; 4];
    let resp_ok = b"OK";
    let resp_nf = b"NF";
    let resp_err_shard = b"ES";
    loop {
        if reader.read_exact(&mut header).await.is_err() { break; }
        let op = header[0];
        let key_len = header[1] as usize;
        let val_len = u16::from_be_bytes([header[2], header[3]]) as usize;
        let total_len = key_len + val_len;
        if total_len > MAX_PAYLOAD_BUFFER { break; }
        if reader.read_exact(&mut buffer[..total_len]).await.is_err() {
            break;
        }
        let key_buf = &buffer[..key_len];
        let shard_id = get_shard_id(key_buf, *num_shards);
        if shard_id != core_id {
            match sender.send_to(shard_id, 125u8).await {
                Ok(data) => {
                    sto
                },
                Err(err) => {
                }
            }
            if writer.write_all(resp_err_shard).await.is_err() {break;}
        } else {
            storage::process(db, storage::Request{
                op: op,
                key: key_buf.to_vec()
            });
        }
        if reader.buffer().is_empty() {
             if writer.flush().await.is_err() { break; }
        }
    }
}
