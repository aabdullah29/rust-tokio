async fn computation1() -> String {
    // .. computation
    "computation 1".to_string()

}

async fn computation2() -> String {
    // .. computation
    "computation 2".to_string()
}

#[tokio::main]
async fn main() {
    let out = tokio::select! {
        res1 = computation1() => res1,
        res2 = computation2() => res2,
    };

    println!("Got = {}", out);
}