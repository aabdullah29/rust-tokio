
In the `tokio-hello-world` project we have two folders `src` and `examples`.
- `src` folder contains the main rust file that will execute
- `examples` folder contains the all examples

we can copy the any example's code into `src/main.rs` and then execute

#### install server: 
`cargo install mini-redis`

#### run server:
`mini-redis-server`


## tokio-hello-world

Explain `tokio-hello-world` [here](https://tokio.rs/tokio/tutorial/hello-tokio)

copy code from `examples/tokio-1-async-await.rs` and past into `src/main.rs`
copy code from `examples/tokio-2-hello-world.rs` and past into `src/main.rs`

run using `cargo run`

or run project:

`cargo run --example tokio-1-async-await`
`cargo run --example tokio-2-hello-world`




## tokio-spawning

Explain `tokio-spawning` [here](https://tokio.rs/tokio/tutorial/spawning)

Spawn is like a nickname of `Task` and it's also act as a speret `Task` in rust.

spawn: use for multiple threads it's get clouser function and return JoinHandle

A Tokio task is an asynchronous green thread. They are created by passing an async block to `tokio::spawn`. The `tokio::spawn` function returns a `JoinHandle`, which the caller may use to interact with the spawned task. The async block may have a return value. The caller may obtain the return value using `.await` on the `JoinHandle` i.e:

```
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
```



Create a server code using `TcpListener::bind()` method to listen the client request and use this code as server and run the previous program `tokio-hello-world` for client. we run both program on different terminal.

#### use simple spawn:
`cargo run --example tokio-3-spawning`

##### terminal 1: 
run `cargo run --example tokio-4-spawining-basics`

##### terminal 2: 
run `cargo run --example tokio-2-hello-world`


Get Error `Error: "unimplemented"` on terminal 2 and get `GOT: Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])` on terminal 1

This server just habdle the one client request at a time becouse the `.await` will block the task until get some response from future. So `tokio::spawn(async {})` will create the a seperet task for each client request and handle the all requests concurrently.

Implement `tokio::spawn(async {})` and also use `let mut db = HashMap::new();` in `tokio-5-spawning-store-data.rs` 


##### terminal 1: 
run `cargo run --example tokio-5-spawning-store-data`

##### terminal 2: 
run `cargo run --example tokio-2-hello-world`







## tokio-shared-state

Explain `tokio-shared-state` [here](https://tokio.rs/tokio/tutorial/shared-state)

`std::sync::Mutex` and `tokio::sync::Mutex` is used to guard the `HashMap` or shared data. A common error is to unconditionally use `tokio::sync::Mutex` from within async code. An async mutex is a mutex that is locked across calls to `.await`, which means thet when we use `tokio::sync::Mutex` then we use `.await` to get the value and that will lock the thread and lock the shared space and other thread can not access it and it's raise the os deadlock state.

A synchronous mutex will block the current thread when waiting to acquire the lock. This, in turn, will block other tasks from processing. However, switching to `tokio::sync::Mutex` usually does not help as the asynchronous mutex uses a synchronous mutex internally.

As a rule of thumb, using a synchronous mutex from within asynchronous code is fine as long as contention remains low and the lock is not held across calls to `.await.` Additionally, consider using `parking_lot::Mutex` as a faster alternative to `std::sync::Mutex`.




##### terminal 1: 
run `cargo run --example tokio-6-shared-state-use-mutex`

##### terminal 2: 
run `cargo run --example tokio-2-hello-world`







## tokio-channels

Explain `tokio-channels` [here](https://tokio.rs/tokio/tutorial/channels)

Channels are use for transfer data between different tasks/threads.

Tokio provides a number of channels, each serving a different purpose.

1. `mpsc`: multi-producer, single-consumer channel. Many values can be sent.
2. `oneshot`: single-producer, single consumer channel. A single value can be sent.
3. `broadcast`: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
4. `watch`: single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.


Create a new channel:
```
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
```

In This example write the client side code and use two type of channels `mpsc` and `oneshot` that will passing the message using the channels. <br>
So we first define the message type using the enum `Command` that will take two commands `Set` and `Get`. <br>
Then create a new channel with a capacity of at most 32 `let (tx, mut rx) = mpsc::channel(32);`

The channel is created with a capacity of 32. If messages are sent faster than they are received, the channel will store them. Once the 32 messages are stored in the channel, calling send(...).await will go to sleep until a message has been removed by the receiver.

The mpsc channel is used to send commands to the task managing the redis connection. The multi-producer capability allows messages to be sent from many tasks. Creating the channel returns two values, a sender and a receiver. The two handles are used separately. They may be moved to different tasks.

After creating channel clone the `tx` and create 2 spawn for create two commands. These two spawn will create two command for client.

```
let tx2 = tx.clone();

// Spawn two tasks, one gets a key, the other sets a key
let t1 = tokio::spawn(async move {
    let cmd = Command::Get {
        key: "hello".to_string(),
    };

    tx.send(cmd).await.unwrap();
});

let t2 = tokio::spawn(async move {
    let cmd = Command::Set {
        key: "foo".to_string(),
        val: "bar".into(),
    };

    tx2.send(cmd).await.unwrap();
});
```


Next, spawn a task or create a new manager spawn that first established a client connection to Redis. Then, that get all messages from the channel which will be sent from first two spawns and received commands are issued via the Redis connection.

```
use mini_redis::client;
// The `move` keyword is used to **move** ownership of `rx` into the task.
let manager = tokio::spawn(async move {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Start receiving messages
    while let Some(cmd) = rx.recv().await {
        use Command::*;

        match cmd {
            Get { key } => {
                client.get(&key).await;
            }
            Set { key, val } => {
                client.set(&key, val).await;
            }
        }
    }
});
```

The final step is to receive the response back from the manager task. The GET command needs to get the value and the SET command needs to know if the operation completed successfully.



### run `tokio-7-channel-use-mpsc.rs`
use `mpsc` in the following program

##### terminal 1: 
run `cargo run --example tokio-6-shared-state-use-mutex`

##### terminal 2: 
run `cargo run --example tokio-7-channel-use-mpsc`




### run `tokio-8-channel-use-oneshot.rs`
use `oneshot` in the following program

##### terminal 1: 
run `cargo run --example tokio-6-shared-state-use-mutex`

##### terminal 2: 
run `cargo run --example tokio-8-channel-use-oneshot`








## tokio I/O

Explain `tokio I/O` [here](https://tokio.rs/tokio/tutorial/io)

Tokio do the almost same things as `std` do in I/O operation but asynchronously. It's use 2 trait `AsyncRead` and  `AsyncWrite` These two traits provide the facilities to asynchronously read from and write to byte streams. These traits have appropriate implementation for `TcpStream`, `File` and `Stdout`. These traits are mostlu use `Vec<u8>` and `&[u8]` byte arrays where a reader or writer is expected.

These traits and their methods can be used through the utility methods provided by `AsyncReadExt` and `AsyncWriteExt` i.e `AsyncReadExt::read` provides an async method for reading data into a buffer, returning the number of bytes read and `AsyncReadExt::read_to_end` reads all bytes from the stream until EOF.

```
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```

`AsyncWriteExt::write` writes a buffer into the writer, returning how many bytes were written and `AsyncWriteExt::write_all` writes the entire buffer into the writer.

```
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}
```

just like `std`, the `tokio::io` module contains a number of helpful utility functions as well as APIs for working with `standard input`, `standard output` and `standard error`. For example, `tokio::io::copy` asynchronously copies the entire contents of a reader into a writer.



### `tokio-9-echo-server-copy-using-io-copy.rs`
The echo server binds a TcpListener and accepts inbound connections in a loop. For each inbound connection, data is read from the socket and written immediately back to the socket. The client sends data to the server and receives the exact same data back.

run `cargo run --example tokio-9-echo-server-copy-using-io-copy`
run `cargo run --example tokio-11-echo-client`


### `tokio-10-echo-server-copy-manually.rs`
To do this, we use `AsyncReadExt::read` and `AsyncWriteExt::write_all`.

run `cargo run --example tokio-10-echo-server-copy-manually`
run `cargo run --example tokio-11-echo-client`






## tokio Framing

Explain `tokio Framing` [here](https://tokio.rs/tokio/tutorial/framing)

Framing is the process of taking a byte stream and converting it to a stream of frames. A frame is a unit of data transmitted between two peers. 











## tokio async in depth

Explain `tokio async in depth` [here](https://tokio.rs/tokio/tutorial/async)

 Asynchronous Rust takes a unique approach and asynchronous Rust functions return futures. Futures must have poll called on them to advance their state. Rust futures are state machines that manage the state of any function/task like `Poll::Ready` and `Poll::Pending` etc.

### Futures




implement the future from `std::future::futureFuture`

run `cargo run --example tokio-12-async-impl-future`


implement future and resolve the future manually (without using await)

run `cargo run --example tokio-13-async-resolve-future-manually`




### Executors

Asynchronous Rust functions return futures. Futures must have poll called on them to advance their state. Futures are composed of other futures. So, the question is, what calls poll on the very most outer future?

`.await` always call in a `async` and executer is the program that execute the first `Future` or `async` function call.
use `futures = "0.3"` in cargo and `use futures::task;` for execute the future in sepret task.

main is not async in this example.

run `cargo run --example tokio-14-async-executor`




### Waker

run `cargo run --example tokio-15-async-waker`


#### Update the MiniTokio:

Wakers are Sync and can be cloned. When wake is called, the task must be scheduled for execution. To implement this, we have a channel. When the wake() is called on the waker, the task is pushed into the send half of the channel. Our Task structure will implement the wake logic. To do this, it needs to contain both the spawned future and the channel send half.

use `crossbeam = "0.8"` in cargo and `use crossbeam::channel;` for `Send` and `Sync` the waker, as the standard library channel is not Sync.

run `cargo run --example tokio-16-async-update-waker`



### mini-tokio instance

run `cargo run --example tokio-17-mini-tokio-instance`







## tokio Select

when multiple `Futures` call then the code run in sequence.  
So, when we wanted to add concurrency, we spawned a new task. It's a additional ways to concurrently execute asynchronous code with Tokio.

The `tokio::select!` macro allows waiting on multiple async computations and returns when a single computation completes.

The select! macro can handle more than two branches. The current limit is 64 branches. Each branch is structured as:

`<pattern> = <async expression> => <handler>`

For example:

run `cargo run --example tokio-18-select-example`

run `cargo run --example tokio-19-select-return-example`


### select-use-else
In the below example, the select! expression waits on receiving a value from rx1 and rx2. If a channel closes, recv() returns None. This does not match the pattern and the branch is disabled. The select! expression will continue waiting on the remaining branches.

Notice that this select! expression includes an else branch. The select! expression must evaluate to a value. When using pattern matching, it is possible that none of the branches match their associated patterns. If this happens, the else branch is evaluated.


run `cargo run --example tokio-20-select-use-else`



### select-borrowing
When spawning tasks, the spawned `async` expression must own all of its data. The `select!` macro does not have this limitation. Each branch's `async` expression may `borrow` data and operate `concurrently`. Following Rust's `borrow` rules, multiple async expressions may immutably borrow a single piece of data or a single async expression may mutably borrow a piece of data.

In the below example the data variable is being borrowed `immutably` from both async expressions. When one of the operations completes successfully, the other one is dropped. Because we pattern match on `Some(v)`, if an expression fails, the other one continues to execute.

When it comes to each branch's `<handler>`, `select!` guarantees that only a single `<handler>` runs. Because of this, each `<handler>` may mutably borrow the same data.

run `cargo run --example tokio-21-select-borrowing`



### select-with-loop
The select! macro randomly picks branches. The select! macro is often used in loops. This section will go over some examples to show common ways of using the select! macro in a loop. We start by selecting over multiple channels.

This example selects over the three channel receivers. When a message is received on any channel, it is written to STDOUT. When a channel is closed, recv() returns with None. By using pattern matching, the select! macro continues waiting on the remaining channels. When all channels are closed, the else branch is evaluated and the loop is terminated.

If rx1 always contained a new message, the remaining channels would never be checked.

If when select! is evaluated, multiple channels have pending messages, only one channel has a value popped. All other channels remain untouched, and their messages stay in those channels until the next loop iteration. No messages are lost.

run `cargo run --example tokio-22-select-with-loop`





















