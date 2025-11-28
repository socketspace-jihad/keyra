
use std::fs::read;

use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, net::TcpStream, task};


static MAX_KEY_LENGTH: usize = 1000;
static MAX_VALUE_LENGTH: usize = 1000;

pub async fn handle_read_vectored(mut stream: TcpStream){
    let mut reader = BufReader::new(stream);
    loop {
        let buf_ref = reader.fill_buf().await.unwrap(); 

        if buf_ref.is_empty() {
            break;
        }

        if buf_ref.len() >= 4 {
            println!("{:?}",&buf_ref[..4]);
            reader.consume(4);
            continue;
        }
    }
}

