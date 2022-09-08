use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};


#[tokio::main]
async fn main() {

    // create a listener port using TcpListener
    // TcpListener return Future
    // that Future return Result
    // and thst result return TcpListener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // we get socket from TcpListener using .accept() method
    // .accept() return a resutl of tuple of socket and address
    let (mut socket, _address) = listener.accept().await.unwrap();
    // let (tx, _rx) = broadcast::channel(10);

    println!("\nAccept Client Connection..!");


    loop {
        let mut buffer = [0u8; 1024];

        // read the console input and save it in a buffer
        // .read is associated with tokio::io::AsyncReadExt
        // .read read the input buffer and write it on our buffer(u8 array)
        // and return the input buffer size 
        let bytes_read_size = socket.read(&mut buffer).await.unwrap();
        // println!("Read {} bytes..!", bytes_read_size);
    

        println!("Get msg:  {}", std::str::from_utf8(&buffer[..bytes_read_size]).expect("Error: not convert to string."));
        // socket.write_all(&buffer[..bytes_read_size]).await.unwrap();  


        // get input from console
        let mut console_buffer = String::new();
        std::io::stdin().read_line(&mut console_buffer).expect("Error: user input.");
        console_buffer = format!("{}{}{}", "Client Get msg: ",console_buffer, "Client Send msg: ");



        // receive data from buffer and send it to console
        // .write_all is associated with tokio::io::AsyncWriteExt
        // .write_all write our buffer(u8 array) data on output buffer
        // we write only those bytes which have some data so we use bytes_read_size
        socket.write_all(console_buffer.as_bytes()).await.unwrap();    
    }

}
