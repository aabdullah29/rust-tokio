use tokio::sync::mpsc::{channel,Sender,Receiver};

use crate::time::sleep;
use crate::message::Message;

/*
receive some messages
*/
pub async fn file_sink(mut channel: Receiver<Message>){
    while let Some(msg)= channel.recv().await {
        println!("write to file: {:?}", msg);
    }
}