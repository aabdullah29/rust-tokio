In this example we create a new file `message_subject_scanner.rs` and `message_recorder_spawner.rs`.

## Changes
1. `Cargo.toml` file <br>
**tokio use for async operations** <br>
tokio = {version="0.2.16", features=["rt-core","time","macros","sync"]} <br>
**failure use for error messages (return result error)** <br>
failure = "0.1.7" <br>
**bincode use for** <br>
bincode = "1.2.1" <br>
**serde use for serialization, deserialization (can also use for json)** <br>
serde = { version = "1.0.106", features = ["derive"] } <br>
**use for generate the random number** <br>
rand = "0.7.3" <br>

2. `message_subject_scanner.rs`


3. `message_recorder_spawner.rs`