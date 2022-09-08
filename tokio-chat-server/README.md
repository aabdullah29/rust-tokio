Create a server that will wait for client request and if it get any request times then giv the responce to client.

use `tokio::net::TcpListener::bind("127.0.0.1:6379").await` for get incomming client requests, 
- its return the `impl Future<Result<TcpListener,Error>>` so we use `.await` to resolve the `Future`
- after resolve the future it's return the `Result<TcpListener,Error>` so we use `.unwrap()` to get `TcpListener` result

Then we will ready to accept `TCP` connections

## 1. Run `tokio-basics-chat-server.rs` Code:
- 1st go into `tokio-chat-server` folder then run the code `cargo run --example tokio-basics-chat-server`
- Then open 2nd terminal and run `telnet localhost 8080` then write any message

message will show in `terminal 1` and write replay on `terminal 1` that will show on terminal 2






## 2. Run `tokio-chat-server-use-buf-reader.rs` Code:
In this program we use  `BufReader` for get user input bytes buffer and  `AsyncBufReadExt` for read data from socket buffer.
also use the `socket.split()` method to split the socket into two parts 1st for `reader` and 2nd for `writter`

- 1st go into `tokio-chat-server` folder then run the code `cargo run --example tokio-chat-server-use-buf-reader`
- Then open 2nd terminal and run `telnet localhost 8080` then write any message

server will send the same message back to client




## 3. Run `tokio-chat-server-multiple-client-using-spawn.rs` Code:

#### Spawn:
when we try to handle multiple client request then our `.await` will block our task (it's not blosk our thread it's block the task). Task is a unit of work in async world it's a very light waight thread, so we use `tokio::spawn`, this is the smart task (It's not an os thread or os task it's just unit of asynchronous programming) tokio use it very effeciently and run multiple tasks simultaneously.

Spawn take a `clouser function` as perameter and run them in parallel. we use `async move {}` it'a an async block work like `async function`.


In this program we use  `tokio::spawn` for handle multiple client requests and past our code in `async move {}` whic is an async block and use an other `loop` for handle multiple requests.

- 1st go into `tokio-chat-server` folder then run the code `cargo run --example tokio-chat-server-multiple-client-using-spawn`
- Then open 2nd terminal and run `telnet localhost 8080` then write any message
- Then open 3rd terminal and run `telnet localhost 8080` then write any message

server will send the same message back to that same client







## 4. Run `tokio-chat-server-use-channel-for-multi-clients.rs` Code:

#### channel:
When we want to send message from one task to other task then we need a shared space, and for avoiding data race conditions on shared space we use `std::sync::Arc` for multipule owners and `std::sync::Mutex;` for safely share data between tasks/threads.

A channel is an easy way to share data between many threads. They are fairly popular because they are pretty simple to put together. Rust channels are available in `std::sync::mpsc::channel`, mpsc means "multiple producer, single consumer", so "many threads sending to one place". Channel can create using `channel()`. This Return a tuple of  `(sender, receiver)` that are tied together. 

In this program we use tokio broadcast channel that available in `tokio::sync::broadcast;` . It's a special kind of channel which allow multiple consumers and multiple producer to use the same channel. it's get a `usize` intiger as input perameter for creating channels. This number tell the channel que size.

so write this line`let (tx, rx) = broadcast::channel::<String>(10);` which means, create a tokio broadcast channel of `String` type and it's buffer size will 10.


#### `tokio::select!`:
It's a macro that tokio provides to resolve the `Future` in parallel fassion. It's run multiple asynchronous things at the same time and then work on the first one which get the result first.

In `tokio::select!` macro the `.await` is not use. this macro have 3 parts:
1. Identifier: It's run after resolve the Future and it's get a `Result<Ok, Err>`
2. Future: All Futures in a `tokio::select!` macro will run at same time but which get the result first, it's code will be execute
3. Code Block: Any work which we wand to do after resolve the Future, it's execute after assign the value to it's Identifier

```
tokio::select! {
    identifier (2nd assign the Future result to identifier) = Future (1st resolve the Future) => {
        code block which will run after resolve the future (3rd run the code and use identifier here)
    } 

    result = SomeFuture => {
        print!("{}", result);
    } 
}
```

now all clients will get the message from a client


- 1st go into `tokio-chat-server` folder then run the code `cargo run --example tokio-chat-server-use-channel-for-multi-clients`
- Then open 2nd terminal and run `telnet localhost 8080` then write any message
- Then open 3rd terminal and run `telnet localhost 8080` then write any message

server will send the same message back to client and send to all other clients









## 5. Run `tokio-chat-server-use-socket-addr-in-multy-client.rs` Code:

Use `broadcast::channel::<(String, std::net::SocketAddr)>(10);` instead of `broadcast::channel::<String>(10);` To avoide the get same message back at sender console.

now all clients will get the message from a client


- 1st go into `tokio-chat-server` folder then run the code `cargo run --example tokio-chat-server-use-socket-addr-in-multy-client`
- Then open 2nd terminal and run `telnet localhost 8080` then write any message
- Then open 3rd terminal and run `telnet localhost 8080` then write any message

server will send the message to all other clients like a broadcast




## 6. Run `tokio-chat-server-complete.rs` Code:
This the same `tokio-chat-server-use-socket-addr-in-multy-client.rs` code and just remove the comments


- 1st go into `tokio-chat-server` folder then run the code `cargo run --example tokio-chat-server-complete`
- Then open 2nd terminal and run `telnet localhost 8080` then write any message
- Then open 3rd terminal and run `telnet localhost 8080` then write any message

server will send the message to all other clients like a broadcast







