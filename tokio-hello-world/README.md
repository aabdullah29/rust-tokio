
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

`std::sync::Mutex` and `tokio::sync::Mutex` is used to guard the `HashMap` or shared data. A common error is to unconditionally use `tokio::sync::Mutex` from within async code. An async mutex is a mutex that is locked across calls to `.await`, which means thet when we use `tokio::sync::Mutex` then we use `.await` to get the value and that will lock the thread and lock the shared space and other thread can not access it and it's raise the os deadlock state.

A synchronous mutex will block the current thread when waiting to acquire the lock. This, in turn, will block other tasks from processing. However, switching to `tokio::sync::Mutex` usually does not help as the asynchronous mutex uses a synchronous mutex internally.

As a rule of thumb, using a synchronous mutex from within asynchronous code is fine as long as contention remains low and the lock is not held across calls to `.await.` Additionally, consider using `parking_lot::Mutex` as a faster alternative to `std::sync::Mutex`.




##### terminal 1: 
run `cargo run --example tokio-6-shared-state-use-mutex`

##### terminal 2: 
run `cargo run --example tokio-2-hello-world`







## tokio-channels

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

### run `tokio-7-channel-use-mpsc.rs`

##### terminal 1: 
run `cargo run --example tokio-6-shared-state-use-mutex`

##### terminal 2: 
run `cargo run --example tokio-7-channel-use-mpsc`




### run `tokio-8-channel-use-oneshot.rs`

##### terminal 1: 
run `cargo run --example tokio-6-shared-state-use-mutex`

##### terminal 2: 
run `cargo run --example tokio-8-channel-use-oneshot`



