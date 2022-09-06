In this example we change the `message_generator` function.
now this function will send the message but if we send a new message using `Ctrl enum` then it will receive and send the changing.
we use the `use tokio::select;` and `use tokio::sync::oneshot;` for this purpose

## Changes in
1. `Cargo.toml` file 
**tokio use for async operations**
tokio = {version="0.2.16", features=["rt-core","time","macros","sync"]}
**failure use for error messages**
failure = "0.1.7"

2. `file_sink.rs`
add libraries
`use tokio::sync::mpsc::{channel,Sender,Receiver};`
`use tokio::select;`
`use tokio::sync::oneshot;`

change the `message_generator` method


3. `main.rs`
add the libraries
`use tokio::prelude::*;`
`use tokio::sync::mpsc::{channel};`
`use tokio::sync::oneshot;`
`use failure::{Fallible};`

also change the     `main` method

4. `file_sink.rs`
we create a new function `file_sink_g` this is a genaric function which can get the any type of input and print it on console
