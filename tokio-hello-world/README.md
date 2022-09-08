
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

copy code from `examples/tokio-hello-world.rs` and past into `src/main.rs`
copy code from `examples/tokio-async-await.rs` and past into `src/main.rs`
run using `cargo run`

or run project:

`cargo run --example tokio-hello-world`
`cargo run --example tokio-async-await`



## tokio-spawning

spawn: use for multiple threads it's get clouser function and return JoinHandle

A Tokio task is an asynchronous green thread. They are created by passing an async block to `tokio::spawn`. The `tokio::spawn` function returns a `JoinHandle`, which the caller may use to interact with the spawned task. The async block may have a return value. The caller may obtain the return value using `.await` on the `JoinHandle`.

Now we create a server code using `TcpListener::bind()` method to listen the client request and use this code as server and run the previous program `tokio-hello-world` for client. we run both program on different terminal.

#### use simple spawn:
`cargo run --example tokio-spawning`

##### terminal 1: 
run `cargo run --example tokio-spawining-basics`

##### terminal 2: 
run `cargo run --example tokio-hello-world`


get Error `Error: "unimplemented"`





