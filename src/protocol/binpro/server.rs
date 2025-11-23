use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};


fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    loop {
        let mut buff = String::new();
        match reader.read_line(&mut buff) {
            Ok(data)=>{
                println!("{:?}",buff);
            },
            Err(_)=>{
                break;
            }
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    println!("Listening on 127.0.0.1:4000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tokio::task::spawn(async move{
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

