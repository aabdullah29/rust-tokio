use tokio::sync::mpsc::{Receiver};
use std::fs::File;
use std::io::prelude::*;
use failure::Fallible;
use serde::{Serialize};
// use serde::{Serialize, Deserialize};

/*
make it generic
receive some messages
and write into a file
*/
pub async fn file_sink<T: core::fmt::Debug + Serialize>(filepath: &'static str, mut channel: Receiver<T>) -> Fallible<()>{
    // open the file
    // let mut file = File::create("data.bin").expect("Error in creating file");
    let mut file = File::create(format!("data/{}.data.bin", filepath))//.await
        .expect("Error: cannot open file.");
    
    while let Some(msg)= channel.recv().await {
        println!("Writing to file {} msg {:?}", filepath, msg);
        // write data in binary format to the file
        file.write(&bincode::serialize(&msg)?)//.await
            .expect("Error: cannot write to file.");
    }

    // close the file
    Ok(())
}



/*
receive some messages
*/
// pub async fn file_sink(mut channel: Receiver<Message>){
//     while let Some(msg)= channel.recv().await {
//         println!("write to file: {:?}", msg);
//     }
// }