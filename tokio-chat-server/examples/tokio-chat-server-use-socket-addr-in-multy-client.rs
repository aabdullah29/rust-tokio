use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener,
    sync::broadcast,
};



#[tokio::main]
async fn main() {

    // create a listener port using TcpListener
    // TcpListener return Future
    // that Future return Result
    // and thst result return TcpListener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();


    // use broadcast::channel for multiple consumer and multiple producer in multi clients chat
    // they will pass one client message to other client
    // create a new channel of (string, SocketAddr) type
    let (tx, _rx) = broadcast::channel::<(String, std::net::SocketAddr)>(10);

    // handle multiple client requests 
    loop {

        // clone tx because this life will end after each iteration
        let tx = tx.clone();
        // tx.subscribe() use for receive data from channel
        let mut rx = tx.subscribe();


        // we get socket from TcpListener using .accept() method
        // .accept() return a resutl of tuple of socket and address
        let (mut socket, socket_address) = listener.accept().await.unwrap();
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
                

                // use for resolve the multiple Future in parallel
                tokio::select! {

                    
                    // read the console input and save it in a buffer
                    // use .read_line which associated with tokio::io::AsyncBufReadExt 
                    //instead of => .read ehichis associated with tokio::io::AsyncReadExt
                    // .read_line read the input buffer and write it on string
                    // and return the size 
                    result = reader_buffer.read_line(&mut string_buffer) => {

                        // if did not get any data then end the connection
                        if result.unwrap() == 0 {
                            break;
                        }

                        // write this data on the channel buffer
                        // pass message and socket address
                        tx.send((string_buffer.clone(), socket_address)).unwrap();

                        // clear the input data
                        string_buffer.clear(); 
                    },


                    // receive data from buffer and send it to console
                    result = rx.recv() => {

                        let (msg, msg_address) = result.unwrap();
                        
                        // if the sender address not same then send the message
                        if msg_address != socket_address {

                            // write the same input back and send to client
                            // .write_all is associated with io::AsyncWriteExt
                            // .write_all write our buffer(u8 array) data on output buffer
                            // we write only those bytes which have some data so we use bytes_read_size
                            writter.write_all(msg.as_bytes()).await.unwrap(); 
                        }
                    },
                }
  
            }

        });

    }

}
