use glommio::channels::shared_channel::SharedReceiver;


pub async fn handle_op(receiver: SharedReceiver<u8>, data: u8){
    let conn = receiver.connect().await; 
    loop {
        match conn.recv().await {
            Some(data) => {
                println!("GET DATA {}",data);
            },
            None => {
                break;
            }
        }
    }
}
