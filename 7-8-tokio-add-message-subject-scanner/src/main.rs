#![allow(dead_code, unused_imports, unused_variables)]

use failure::{Fallible};

mod message;
mod file_sink;

mod time;
use time::sleep;

mod message_generator;
mod message_recorder;

mod message_subject_scanner;
use message_subject_scanner::{MessageSubjectScanner};

mod message_recorder_spawner;
use message_recorder_spawner::{MessageRecorderSpawner};

#[tokio::main]
async fn main() -> Fallible<()> {
    let msg_sunject_scanner = MessageSubjectScanner::spawn();
    let msg_recorder_spawner = MessageRecorderSpawner::spawn(msg_sunject_scanner)?;

    sleep(20000).await; // print 20 messages
    println!("Exiting program");
    Ok(())
}
