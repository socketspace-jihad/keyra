use std::io::{BufRead, BufReader, IoSliceMut, Read, Write};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, task};


static MAX_KEY_LENGTH: usize = 1000;
static MAX_VALUE_LENGTH: usize = 1000;

pub async fn handle_read_vectored(mut stream: TcpStream){
    let mut header = [0u8;1];

    let mut key_length = [0u8;1];
    let mut value_length = [0u8;2];

    let mut payload = vec![0u8;MAX_KEY_LENGTH+MAX_VALUE_LENGTH];

    loop {
        let mut slice = [
            IoSliceMut::new(&mut header),
            IoSliceMut::new(&mut key_length),
            IoSliceMut::new(&mut value_length),
            IoSliceMut::new(&mut payload)
        ];

        match stream.try_read_vectored(&mut slice) {
            Ok(size) if size > 0 =>{
                let key_idx = u8::from_be_bytes(key_length) as usize;
                let _ = str::from_utf8(&payload[0..key_idx]).unwrap();
                tokio::spawn(async{
                });
            },
            Ok(_) => {
                stream.shutdown().await.unwrap();
                break;
            },
            Err(_)=>{
                stream.shutdown().await.unwrap();
                break;
            }
        }
    }
}

