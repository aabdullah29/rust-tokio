use tokio::sync::mpsc::{channel,Sender,Receiver};
use std::fs::File;
use std::io::prelude::*;
use failure::Fallible;
use serde::{Serialize, Deserializer};


use crate::time::sleep;
use crate::message::Message;


/*
make it generic
receive some messages
and write into a file
*/
pub async fn file_sink_g<T: core::fmt::Debug + Serialize>(mut channel: Receiver<T>) -> Fallible<()>{
    // open the file
    // let mut file = File::create("data.bin").expect("Error in creating file");
    let mut file = File::create("data.bin")?;
    while let Some(msg)= channel.recv().await {
        // write data in binary format to the file
        file.write(&bincode::serialize(&msg)?)?; 
    }
    // close the file

    Ok(())
}



/*
receive some messages
*/
pub async fn file_sink(mut channel: Receiver<Message>){
    while let Some(msg)= channel.recv().await {
        println!("write to file: {:?}", msg);
    }
}