#![allow(dead_code, unused_imports, unused_variables)]

use failure::{Fallible};

mod message;
mod file_sink;

mod time;
use time::sleep;

mod message_generator;
use message_generator::{Ctrl};

mod message_recorder;
use message_recorder::{MessageRecorder};

#[tokio::main]
async fn main() -> Fallible<()>{
    // println!("Hello, world!");
    let mut msg_recorder = MessageRecorder::spawn()?;

    sleep(1000).await; // print mssages for 1 seconds    
    println!("Health message send..!");
    let response = msg_recorder.send_ctrl_msg(Ctrl::Health).await.expect("Error in 'ctx.send(Ctrl::Health(otx))'");
    println!("Received helth responce is: {:?}", response);
    sleep(1000).await;

    println!("Quit message send..!");
    let response = msg_recorder.send_ctrl_msg(Ctrl::Quit).await.expect("Error in 'ctx.send(Ctrl::Quit)'");
    println!("Quit message esponce is: {:?}", response);
    sleep(5000).await;

    println!("\nExit Program!");
    Ok(())
}
