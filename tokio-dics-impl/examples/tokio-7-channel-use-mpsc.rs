use bytes::Bytes;
use tokio::sync::mpsc;
use mini_redis::client;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}





#[tokio::main]
async fn main() {


    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);

    // ... Rest comes here

    // The `Sender` handles are moved into the tasks. As there are two

    // tasks, we need a second `Sender`.
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "foo".to_string(),
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








    
    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    let r = client.get(&key).await;
                    println!("Get: {:?}", r);
                }
                Set { key, val } => {
                    let r = client.set(&key, val).await;
                    println!("Set: {:?}", r);
                }
            }
        }
    });


    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}


