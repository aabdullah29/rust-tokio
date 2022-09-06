In this example we change the `file_sink_g` function.
now this function will create a new file if it's not exist and write the message 
into the file. but before write the message this function will serialize the messase into the binary format.

## Changes
1. `Cargo.toml` file <br>
**tokio use for async operations** <br>
tokio = {version="0.2.16", features=["rt-core","time","macros","sync"]} <br>
**failure use for error messages** <br>
failure = "0.1.7" <br>
**bincode use for binary format** <br>
bincode = "1.2.1" <br>
**serde use for serialization, deserialization (can also use for json)** <br>
serde = { version = "1.0.106", features = ["derive"] } <br>

2. `message.rs`
add the library and its drive macros
`use serde::{Serialize, Deserialize};`
`#[derive(Debug, Serialize, Deserialize)]`


3. `file_sink.rs`
add libraries
`use tokio::sync::mpsc::{channel,Sender,Receiver};`
`use std::fs::File;`
`use std::io::prelude::*;`
`use failure::Fallible;`
`use serde::{Serialize, Deserializer};`

change the `file_sink_g` method