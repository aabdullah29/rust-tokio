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