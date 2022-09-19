use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello Future..!");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}



// work like a state machine
// resolve feture manually
enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        use MainFuture::*;

        loop {
            match *self {
                State0 => {
                    let when = Instant::now() +
                        Duration::from_millis(1);
                    let future = Delay { when };
                    *self = State1(future);
                    println!("State0...!");
                }
                State1(ref mut my_future) => {
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            println!("State1 is Done...!");
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            print!("Pending... ");
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    println!("Terminated...!");
                    panic!("future polled after completion")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // let when = Instant::now() + Duration::from_millis(10);
    // let future = Delay { when };

    // let out = future.await;
    // assert_eq!(out, "done");

    let result = MainFuture::State0.await;
    println!("{:?}", result);
}