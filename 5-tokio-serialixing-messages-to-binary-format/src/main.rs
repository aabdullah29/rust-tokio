#![allow(dead_code, unused_imports, unused_variables)]
use tokio::prelude::*;
use tokio::sync::mpsc::{channel};
use tokio::sync::oneshot;
// use failure::{Fallible};


mod message;
use message::Message;

mod time;
use time::sleep;

mod message_generator;
use message_generator::{message_generator, Ctrl};

mod file_sink;
use file_sink::{file_sink, file_sink_g};


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let (tx, rx) = channel::<Message>(10);
    let (mut ctx, crx) = channel::<Ctrl>(10);

    // send and receive messages
    tokio::spawn(message_generator(crx, tx));
    // tokio::spawn(file_sink(rx));
    tokio::spawn(file_sink_g(rx));

    sleep(1000).await; // print mssages for 1 seconds

    
    let (otx, orx) = oneshot::channel();
    println!("Health message send..!");
    ctx.send(Ctrl::Health(otx)).await.expect("Error in 'ctx.send(Ctrl::Health(otx))'");
    sleep(1000).await;
    let response = orx.await;
    println!("Received helth responce is: {:?}", response);

    println!("Quit message send..!");
    ctx.send(Ctrl::Quit).await.expect("Error in 'ctx.send(Ctrl::Quit)'");
    sleep(5000).await;

    println!("\nExit Program!");
}
