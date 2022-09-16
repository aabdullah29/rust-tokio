async fn action(input: Option<i32>) -> Option<String> {
    println!("=> action: {:?}", input);
    // If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic here
    Some(format!("{}", i))
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);
    
    tokio::spawn(async move {
        for i in 0..10 {
            let _ = tx.send(1+(i*10)).await;
            let _ = tx.send(3+(i*10)).await;

            if i > 5 {
                let _ = tx.send(2+(i*10)).await;
            }
        }
        println!("End spwn..!");
    });


    let mut done = true;
    let operation = action(None);
    tokio::pin!(operation);
    

    
    loop {
        tokio::select! {
            res = &mut operation, if done => {
                println!("=> operation: {:?}", res);
                done = false;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                println!("select recv: {}", v);

                if v % 2 == 0 {
                    // `.set` is a method on `Pin`.
                    operation.set(action(Some(v)));
                    done = true;
                }
            }
        }
    }
}