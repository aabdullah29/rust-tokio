use tokio::{
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener,
    sync::broadcast,
};



#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<(String, std::net::SocketAddr)>(10);

    loop {

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let (mut socket, socket_address) = listener.accept().await.unwrap();
        println!("\nAccept Client Connection..!");

        tokio::spawn( async move {
            
            let (reader,mut  writter) = socket.split();
            let mut reader_buffer = BufReader::new(reader);

            loop {

                let mut string_buffer = String::new();
                
                tokio::select! {

                    result = reader_buffer.read_line(&mut string_buffer) => {

                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((string_buffer.clone(), socket_address)).unwrap();
                        string_buffer.clear(); 
                    },

                    result = rx.recv() => {

                        let (msg, msg_address) = result.unwrap();
                        
                        if msg_address != socket_address {
                            writter.write_all(msg.as_bytes()).await.unwrap(); 
                        }
                    },
                }
            }
        });
    }
}
