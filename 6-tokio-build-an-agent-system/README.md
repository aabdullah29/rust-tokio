In this example we change the `message_generator` file.
and we create a struct `MessageRecorder` and implement it.
and we send and receive messages from that struct using these line which we were write in `main.rs`
1. `let (tx, rx) = channel::<Message>(10);`
2. `let (ctx, crx) = channel::<Ctrl>(10);`
and we implenet `send_ctrl_msg` function for send control message which we were sned in `main.rs`


we do this example in 2 parts:

# Changes part 1
In part 1 we just create new file `message_recorder.rs` and write the same code that is show in point no.2.
and we also do some changes in `main.rs` file that can find in point no.3.
and we did't change the `message_generator.rs` file in step 1 but we change it in step 2 so anyone can find the privious code of this file in point no.4.

1. `Cargo.toml` file <br>
**tokio use for async operations** <br>
tokio = {version="0.2.16", features=["rt-core","time","macros","sync"]} <br>
**failure use for error messages** <br>
failure = "0.1.7" <br>
**bincode use for binary format** <br>
bincode = "1.2.1" <br>
**serde use for serialization, deserialization (can also use for json)** <br>
serde = { version = "1.0.106", features = ["derive"] } <br>

2. `message_recorder.rs`
```
use tokio::sync::mpsc::{channel,Sender};
use failure::{Fallible};


use crate::message_generator::{message_generator, Message, Ctrl};
use crate::file_sink::{file_sink_g};

pub struct MessageRecorder {
    ctrl: Sender<Ctrl> 
}

impl MessageRecorder {
    /// Spawns a MessageRecorder, will spawn
    /// two agents. A message generator and a file_sink
    /// connect the two, and will return a handle to
    /// the agents in the form of a control channel
    pub fn spawn() -> Fallible<MessageRecorder> {
        let (tx, rx) = channel::<Message>(10);
        let (ctx, crx) = channel::<Ctrl>(10);

        // message_generation -> file_sink
        tokio::spawn(message_generator(crx, tx));
        tokio::spawn(file_sink_g(rx));

        Ok(MessageRecorder { ctrl: ctx })
    }

    /// Sends a ctrl message to the spawned agents.
    pub async fn send_ctrl_msg(&mut self, msg: Ctrl) -> Fallible<()> {
        self.ctrl.send(msg).await?; 
        Ok(())
    }

}
```


3. `main.rs`
```
#![allow(dead_code, unused_imports, unused_variables)]

use tokio::sync::oneshot;
use failure::{Fallible};


mod message;
mod file_sink;

mod time;
use time::sleep;

mod message_generator;
use message_generator::{Ctrl};

mod message_recorder;
use message_recorder::{MessageRecorder};

#[tokio::main]
async fn main() -> Fallible<()>{
    // println!("Hello, world!");
    let mut msg_recorder = MessageRecorder::spawn()?;

    sleep(1000).await; // print mssages for 1 seconds    
    println!("Health message send..!");
    let (otx, orx) = oneshot::channel();
    msg_recorder.send_ctrl_msg(Ctrl::Health(otx)).await.expect("Error in 'ctx.send(Ctrl::Health(otx))'");
    sleep(1000).await;
    let response = orx.await;
    println!("Received helth responce is: {:?}", response);

    println!("Quit message send..!");
    msg_recorder.send_ctrl_msg(Ctrl::Quit).await.expect("Error in 'ctx.send(Ctrl::Quit)'");
    sleep(5000).await;

    println!("\nExit Program!");
    Ok(())
}
```

4. `message_generator.rs`
```
use tokio::sync::mpsc::{Sender,Receiver};

use tokio::select;
use tokio::sync::oneshot;

pub use crate::time::sleep;
pub use crate::message::Message;


/*
Ctrl channel
Quit
Health -> Health or not (send message again using oneshot)
*/
#[derive(Debug)]
pub enum Ctrl{
    Quit,
    Health(oneshot::Sender<HelthRsponse>)
}

#[derive(Debug)]
pub enum HelthRsponse{
    Healthy,
    Unhealthy,
}

/*
send some messages
*/
pub async fn message_generator(mut ctrl: Receiver<Ctrl>, mut channel: Sender<Message>) {
    loop {
        select! {
            msg = channel.send(Message::Hello) => match msg {
                Ok(()) => sleep(100).await,
                Err(_) => {
                    eprintln!("Error in sending message");
                    break;
                }
            },
            ctl = ctrl.recv() => match ctl{
                Some(Ctrl::Quit) => {
                    eprintln!("End by Ctrl::Quit.");
                    break
                },
                Some(Ctrl::Health(rtx)) => {
                    rtx.send(HelthRsponse::Healthy).unwrap()
                },
                None => {
                    eprintln!("Error by Ctrl::Quit.");
                    break
                }
            },
        }
    }
    println!("Message Generator Stope !!")
}
```


# Changes part 2
In step 2 we do some more change like we create some more enums in `message_generator.rs` and we do some change in `main.rs`, `message_recorder.rs` and `message_generator.rs` files

1. `main.rs`
```
#![allow(dead_code, unused_imports, unused_variables)]

use failure::{Fallible};

mod message;
mod file_sink;

mod time;
use time::sleep;

mod message_generator;
use message_generator::{Ctrl};

mod message_recorder;
use message_recorder::{MessageRecorder};

#[tokio::main]
async fn main() -> Fallible<()>{
    // println!("Hello, world!");
    let mut msg_recorder = MessageRecorder::spawn()?;

    sleep(1000).await; // print mssages for 1 seconds    
    println!("Health message send..!");
    let response = msg_recorder.send_ctrl_msg(Ctrl::Health).await.expect("Error in 'ctx.send(Ctrl::Health(otx))'");
    println!("Received helth responce is: {:?}", response);
    sleep(1000).await;

    println!("Quit message send..!");
    let response = msg_recorder.send_ctrl_msg(Ctrl::Quit).await.expect("Error in 'ctx.send(Ctrl::Quit)'");
    println!("Quit message esponce is: {:?}", response);
    sleep(5000).await;

    println!("\nExit Program!");
    Ok(())
}
```

2. `message_recorder.rs`
```
use tokio::sync::mpsc::{channel,Sender};
use tokio::sync::oneshot;
use failure::{Fallible};


use crate::message_generator::{message_generator, Message, Ctrl, CtrlR};
use crate::file_sink::{file_sink_g};

#[derive(Debug)]
pub struct MessageRecorder {
    ctrl: Sender<(Ctrl, oneshot::Sender<CtrlR>)> 
}

impl MessageRecorder {
    /// Spawns a MessageRecorder, will spawn
    /// two agents. A message generator and a file_sink
    /// connect the two, and will return a handle to
    /// the agents in the form of a control channel
    pub fn spawn() -> Fallible<MessageRecorder> {
        let (tx, rx) = channel::<Message>(10);
        let (ctx, crx) = channel::<(Ctrl, oneshot::Sender<CtrlR>)>(10);

        // message_generation -> file_sink
        tokio::spawn(message_generator(crx, tx));
        tokio::spawn(file_sink_g(rx));

        Ok(MessageRecorder { ctrl: ctx })
    }

    /// Sends a ctrl message to the spawned agents.
    pub async fn send_ctrl_msg(&mut self, msg: Ctrl) -> Fallible<CtrlR> {
        let (otx, orx) = oneshot::channel::<CtrlR>();
        self.ctrl.send((msg, otx)).await?; 
        Ok(orx.await?)
    }

}
```

3. `message_generator.rs`
```
use tokio::sync::mpsc::{Sender,Receiver};

use tokio::select;
use tokio::sync::oneshot;

pub use crate::time::sleep;
pub use crate::message::Message;


/*
Ctrl channel
Quit
Health -> Health or not (send message again using oneshot)
*/
/// Control request type
#[derive(Debug)]
pub enum Ctrl{
    Quit,
    Health
}

/// Control response type
#[derive(Debug)]
pub enum CtrlR {
    Quit(QuitResponse),
    Health(HealthR),
}

#[derive(Debug)]
pub enum QuitResponse {
    Ok,
}

#[derive(Debug)]
pub enum HealthR{
    Healthy,
    Unhealthy,
}

/*
send some messages
*/
/// The spawn function of the agent
pub async fn message_generator(mut ctrl: Receiver<(Ctrl, oneshot::Sender<CtrlR>)>, mut channel: Sender<Message>) {
    loop {
        select! {
            msg = channel.send(Message::Hello) => match msg {
                Ok(()) => sleep(100).await,
                Err(_) => {
                    eprintln!("Error in sending message");
                    break;
                }
            },
            ctl = ctrl.recv() => match ctl{
                Some((Ctrl::Quit, rtx)) => {
                    eprintln!("End by Ctrl::Quit.");
                    rtx.send(CtrlR::Quit(QuitResponse::Ok)).expect("Error in 'ctl = ctrl.recv() => Some((Ctrl::Quit, rtx))'");
                    break
                },
                Some((Ctrl::Health, rtx)) => {
                    rtx.send(CtrlR::Health(HealthR::Healthy)).expect("Error in 'ctl = ctrl.recv() => Some((Ctrl::Health, rtx))'")
                    // rtx.send(CtrlR::Health(HealthR::Healthy)).unwrap();
                },
                None => {
                    eprintln!("Error by Ctrl::Quit.");
                    break
                }
            },
        }
    }
    println!("Message Generator Stope !!")
}
```
