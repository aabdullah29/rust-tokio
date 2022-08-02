use tokio::sync::mpsc::{channel,Sender,Receiver};
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