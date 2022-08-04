use tokio::sync::mpsc::{channel, Sender,Receiver};
use tokio::select;
use failure::Fallible;
use rand::prelude::*;

pub use crate::time::sleep;


const VALID_SUBJECTS: &'static [&'static str] = &["orange", "apples", "plumes", "apricot", "mandarin", "pear"];

pub type Subjects = Vec<&'static str>;


// just use Ctrl::Quit 
// just polling here
#[derive(Debug)]
pub enum Ctrl{
    Quit,
}

#[derive(Debug)]
pub struct MessageSubjectScanner {
    ctrl: Sender<Ctrl>,
    subjects: Vec<String>,
    pub subjects_rx: Receiver<Subjects>,
}

impl MessageSubjectScanner {
    pub fn spawn() -> MessageSubjectScanner {
        let (tx, rx) = channel::<Subjects>(10);
        let (ctx, crx) = channel::<Ctrl>(10);

        tokio::spawn(agent_loop(crx, tx));

        MessageSubjectScanner { 
            subjects: Vec::new(),
            ctrl: ctx,
            subjects_rx: rx
        }
    }

    pub async fn send_ctrl_msg(&mut self, msg: Ctrl) -> Fallible<()> {
        self.ctrl.send(msg).await?; 
        Ok(())
    }
}



// use random number generator
fn build_random_subjects() -> Subjects {
    let mut subjects : Vec<&'static str> = VALID_SUBJECTS.into();

    let rng = &mut thread_rng();
    subjects.shuffle(rng);

    subjects.into_iter().take(rng.gen_range(1, VALID_SUBJECTS.len())).collect()
}



// the agent loop
pub async fn agent_loop(mut ctrl: Receiver<Ctrl>, mut channel: Sender<Subjects>) {
    loop {
        let subjects = build_random_subjects();

        select! {
            // Todo: example subject being send, change into randoms
            msg = channel.send(subjects) => match msg {
                Ok(()) => sleep(2000).await,
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
                None => {
                    eprintln!("Error by Ctrl::Quit.");
                    break
                }
            },
        }
    }
    println!("Message Generator Stope !!")
}
