use tokio::sync::mpsc::{channel,Sender,Receiver};

pub use crate::time::sleep;
pub use crate::message::Message;


/*
send some messages
*/
pub async fn message_generator(mut channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(100).await,
            Err(_) => {
                eprintln!("Error in sending message");
                break;
            }
        }
    }
}