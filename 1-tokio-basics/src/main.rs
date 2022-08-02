#![allow(dead_code, unused_imports, unused_variables)]
use tokio::prelude::*;
use tokio::sync::mpsc::{channel,Sender,Receiver};
use tokio::time::{Delay, delay_for};

use std::time::Duration;


/*
sleep for given mini seconds
*/
fn sleep(ms: u64) -> Delay{
    delay_for(Duration::from_millis(ms))
}


#[derive(Debug)]
enum Message{
    Hello,
    World,
}


/*
send some messages
*/
async fn message_generator(mut channel: Sender<Message>) {
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


/*
receive some messages
*/
async fn file_sink(mut channel: Receiver<Message>){
    while let Some(msg)= channel.recv().await {
        println!("write to file: {:?}", msg);
    }
}



#[tokio::main]
async fn main() {
    println!("Hello, world!");


    let (tx, rx) = channel::<Message>(10);
    tokio::spawn(message_generator(tx));
    tokio::spawn(file_sink(rx));

    sleep(2000).await;
    println!("\nHello, world again!");

}
