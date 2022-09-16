use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world2".into()).await.unwrap_or_else(|e| println!("Error: {:?}", e));

    for _ in 0..1000000000 {}
    // Get key "hello"
    let result = client.get("hello").await?;

    println!("\ngot value from the server; result={:?}", result);

    Ok(())
}

