use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);




    tokio::spawn(async move {
        // Do something w/ `tx1` and `tx2`

        let data = "send some data in both. ";

        tokio::select! {
            _ = tx1.send(data) => {

            },
            _ = tx2.send(data) => {

            }
            else => {
                println!("Both channels closed");
            }
        }

    });

    let mut data = String::new();
    tokio::select! {
        Some(v) = rx1.recv() => {
            data = v.to_string();
            println!("Got from rx1: {:?} ", data);
        }
        Some(v) = rx2.recv() => {
            data = v.to_string();
            println!("Got from rx1: {:?} ", data);
        }
        else => {
            data = "else".to_string();
            println!("res =:{:?}", data);
            println!("Both channels closed");
        }
        
    }

    
}