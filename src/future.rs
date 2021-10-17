use crossbeam::channel;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::time::Instant;


// our custom type which we will give a future trait
// so a type has a trait (how easy to understand :) !)
struct Delay {
    when: Instant,
}

impl Future for Delay {
    // dont go deep in for this right now
    type Output = &'static str;

    // poll function basically allows for an executor to poll this future 
    // towards further progress/ completion
    // it takes in a pinned generic mutable type and a context and returns
    // the future's status which is nothing but one of the states of the future's
    // finite state automata
    // context here is an anonymous lifetime
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
