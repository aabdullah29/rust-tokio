async fn action() {
    // Some asynchronous logic
    println!("action..!");
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);    
    
    for i in 0..10 {
        tx.send(format!("Send: {}", i)).await.unwrap();
    }


    let operation = action();
    tokio::pin!(operation);
    
    loop {
        tokio::select! {
            _ = &mut operation => {
                println!("operation..!");
                break
            },
            Some(v) = rx.recv() => {
                println!("Received: {}", v);
            }
        }
    }
}