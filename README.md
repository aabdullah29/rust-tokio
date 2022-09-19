Basics things which belongs to Rust and Tokio [Link](https://tokio.rs/tokio/glossary).


# Futures:
When we want to create an async/await function in rust and write `async` keyword before function then that function run the asynchronously and return a `Future`.
Future is like `promise` in javascript, Future may not have some value on that time but they can get some value in future.
Future frequently use in network reguest because when we create a network call like `gat("google.com")` so this request will take some miniseconds to complete and then give the result, so we use future in this kind of rewuest, when we create a network request then this request will give some `Future` that future may not have some value on this time but it can get some value after some time.
Future maintain their state according ti the request result:

- Pending: Its the 1st state, waiting for transaction to complete
- Complet: Transaction is complete and give some result
- Error: Transaction not complete

Rust now how to generate a `Future` but it can not now how to execute/resolve the `Future`, so we ca fix this using `Tokio`.

### Note:
Future is a trait in rust which have an function signature `poll` that return `enum Poll`, `poll` method run that task again and again until the `Poll` or `Future` state become Ready.


```
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

```
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

### Task, Executor and Reactor in Future
`Task`: is the block of code that will be execut by `Future` or as `Future` <br>
`Executor`: is a sheduler that shedules the `Task`, it's have a `ready-que` of `Tasks` <br>
`Reactor`: it's notify the `Executor` that a `Task` is ready to execute

Rust Future handle all these things in the background. <br>
`Executor`: it's job to repetedly call the `poll` method and pass the `Context` as perameter <br>
`Context`: it's contains the `Reactor` and it's talk to the `Future` if it's resolve then it give the result back otherwise `Reactor` add it again into the `Executor` que, `Reactor` aslo called the `awaker` <br>
`Pin<T>`: it's a `pointer` and the part of function signature. When program run a `Future` or new `Task` than that `Task/Future` occupy some memory in `Heap` and `Pin` point out that memory address. 

In simple worlds we can say that `.await` work as a loop and it's call the `poll` methods until the furure is resolved/Ready
and we can resolve the `Future` with out using `.await` but then we should use a `loop` and `yield` the task until get the Ready state of that task.

# What is Tokio? [Link](https://tokio.rs/tokio/tutorial)

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications. 
Tokio is designed for IO-bound applications where each individual task spends most of its time waiting for IO.

At a high level, Tokio provides a few major components:

- A multi-threaded runtime for executing asynchronous code.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.


## Advantages:

#### Fast:
Tokio is built on top of the async/await language feature, when dealing with networking, there's a limit to how fast you can handle a connection due to latency, so the only way to scale is to handle many connections at once. With the async/await language feature, increasing the number of concurrent operations becomes incredibly cheap, allowing you to scale to a large number of concurrent tasks.

#### Reliable:
Tokio also focuses heavily on providing consistent behaviour with no surprises.

#### Easy:
With tokio we can easily use the rust's async/await feature and write asynchronous code.

#### Flexible:
Tokio provides multiple variations of the runtime. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime. 


## Run Tokio:
For implement tokio wo can use the Redis client and server for implement the async/await features.

#### Redis and mini-redis:
The open source, in-memory data store used by millions of developers as a database, cache, streaming engine, and message broker.
But in this p we will use the [mini-redis](https://github.com/tokio-rs/mini-redis). Mini-Redis is designed with the primary goal of learning Tokio.
Redis [docs](https://redis.io/docs/reference/protocol-spec/) and [commands](https://redis.io/commands/).

#### install server: 
`cargo install mini-redis`

#### run server:
`mini-redis-server`

#### run client on other terminal:
`mini-redis-cli get foo`

#### tokio-hello-world
tokio-hello-world is the first tokio example in which we use `mini-redis` client and server

`tokio-hello-world ` is available in [tokio-docs-impl](./tokio-docs-impl/) project. path: `tokio-docs-impl/examples/tokio-2-hello-world.rs`



## Compile-time green-threading
Rust implements asynchronous programing using a feature called async/await. Functions that perform asynchronous operations are labeled with the async keyword. In our example, the connect function is an `async fn`.

The `async fn` definition looks like a regular synchronous function, but operates asynchronously. Rust transforms the `async fn` at compile time into a routine that operates asynchronously. Any calls to `.await` within the `async fn` yield control back to the thread. The thread may do other work while the operation processes in the background.


The return value of an async fn is an anonymous type that implements the Future trait.

Tokio async main function:
The main function used to launch the application differs from the usual one found in most of Rust's crates.

- It is an `async fn`
- It is annotated with `#[tokio::main]`
An async fn is used as we want to enter an asynchronous context. However, asynchronous functions must be executed by a runtime. The runtime contains the asynchronous task scheduler, provides evented I/O, timers, etc. The runtime does not automatically start, so the main function needs to start it.

The `#[tokio::main]` function is a macro. It transforms the `async fn main()` into a synchronous `fn main()` that initializes a runtime instance and executes the async main function.

Tokio has a lot of functionality (TCP, UDP, Unix sockets, timers, sync utilities, multiple scheduler types, etc).


# Explaination and implementation of Tokio Docs
[tokio-docs-impl](./tokio-docs-impl/) project have a `Readme.md` file in which the basics tokio explaination is availbe and there are also many tokio code examples that can explaina the tokio functionality.




# Some Other Examples:
In this repo we have some examples about tokio and we create sepret project for each concept.
Detail of the projecs are:

### tokio-chat-server
In [tokio-chat-server](./tokio-chat-server/) project we create a chat server that get the `TCP` request from clients.

### 1-tokio-basics
In this project we have just basics use of tokio framework.

### 2-tokio-basics-in-siplit-files
In this project we create sepret files for eact component.

### 3-4-tokio-send-message-using-oneshot
In this project we add some more functionality in `message_generator` file and function, and we also cfeate a genaric function `file_sink_g` in `file_sink` file. 

### 5-tokio-serialixing-messages-to-binary-format
In this project we add some more functionality in `file_sink_g` function in `file_sink` file that will write thw input data in to a binary file in binary format.

### 6-tokio-build-an-agent-system
In this project we add a new file `message_recorder.rs` and we do some new changes in `message_generator.rs` and `main.rs` file

### 7-8-tokio-add-message-subject-scanner
In this project we add 2 new files `message_subject_scanner.rs` and `message_recorder_spawner.rs`.
we also do changes in all project and make it simple and dynamic.
