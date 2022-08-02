#![allow(dead_code, unused_imports, unused_variables)]
use tokio::prelude::*;
use tokio::sync::mpsc::{channel};

mod message;
use message::Message;

mod time;
use time::sleep;

mod message_generator;
use message_generator::message_generator;

mod file_sink;
use file_sink::file_sink;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let (tx, rx) = channel::<Message>(10);
    tokio::spawn(message_generator(tx));
    tokio::spawn(file_sink(rx));

    sleep(2000).await;
    println!("\nHello, world again!");
}
