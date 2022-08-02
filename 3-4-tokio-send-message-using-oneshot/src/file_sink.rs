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



/*
make it generic
receive some messages
*/
pub async fn file_sink_g<T: core::fmt::Debug>(mut channel: Receiver<T>){
    while let Some(msg)= channel.recv().await {
        println!("write to file: {:?}", msg);
    }
}