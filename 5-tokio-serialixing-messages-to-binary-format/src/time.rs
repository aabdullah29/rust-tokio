use tokio::time::{Delay, delay_for};
use std::time::Duration;

/*
sleep for given mini seconds
*/
pub fn sleep(ms: u64) -> Delay{
    delay_for(Duration::from_millis(ms))
}