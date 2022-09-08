use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener,
};


#[tokio::main]
async fn main() {

    // create a listener port using TcpListener
    // TcpListener return Future
    // that Future return Result
    // and thst result return TcpListener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // handle multiple client requests 
    loop {

        // we get socket from TcpListener using .accept() method
        // .accept() return a resutl of tuple of socket and address
        let (mut socket, _address) = listener.accept().await.unwrap();
        // let (tx, _rx) = broadcast::channel(10);
        println!("\nAccept Client Connection..!");


        // use for handle multiple requests concurrently
        // basically it's run this block as resolving the feture
        tokio::spawn( async move {
            
            // tokio socket have two parts: 1st is reader and 2nd is writter
            // split socket reader and writter 
            let (reader,mut  writter) = socket.split();

            // BufReader use for get input stream instead of => let mut buffer = [0u8; 1024];
            let mut reader_buffer = BufReader::new(reader);

            loop {
                // use for get user unput buffer as string
                let mut string_buffer = String::new();
                
                // read the console input and save it in a buffer
                // use .read_line which associated with tokio::io::AsyncBufReadExt 
                // instead of => .read ehichis associated with tokio::io::AsyncReadExt
                // .read_line read the input buffer and write it on string
                // and return the size 
                let bytes_read_size = reader_buffer.read_line(&mut string_buffer).await.unwrap();
                
                // if did not get any data then end the connection
                if bytes_read_size == 0 {
                    break;
                }
            
                // receive data from buffer and send it to console
                // write the same input back and send to client
                // .write_all is associated with io::AsyncWriteExt
                // .write_all write our buffer(u8 array) data on output buffer
                // we write only those bytes which have some data so we use bytes_read_size
                writter.write_all(string_buffer.as_bytes()).await.unwrap(); 


                // clear the input data
                string_buffer.clear();   
            }

        });

    }

}
