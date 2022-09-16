
/*
spawn: use for multiple threads it's get clouser function and return JoinHandle
*/

#[tokio::main]
async fn main() {
    let handle_1 = tokio::spawn(async {
        // Do some async work
        "return value from handle_1"
    });

    // Do some other work

    let handle_2 = tokio::spawn(async {
        // Do some async work
        "return value from handle_2"
    });

    let out = handle_2.await.unwrap();
    println!("\nGOT {}", out);

    let out = handle_1.await.unwrap();
    println!("\nGOT {}", out);
}